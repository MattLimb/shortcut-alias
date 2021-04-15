from os import close
from termcolor import colored
from colorama import Style
import shlex
import subprocess
import re
import os

from .exceptions import RequiredValue
from . import SETTINGS, GLOBAL_ENVIRONMENT

__author__ = "Matt Limb <matt.limb17@gmail.com>"

class Command:
    def __init__(self, **kwargs):
        self.name = kwargs.get("name", None)
        self.description = kwargs.get("description", None)
        self.background = kwargs.get("background", False)
        self.cmd = kwargs.get("cmd", None)
        self.conditionals = kwargs.get("if", [])
        self.mode = kwargs.get("mode", "shell")
        self._verify()

    def _verify(self):
        if not self.name:
            raise RequiredValue("'name' is a required value of a command.")

        if not self.cmd:
            raise RequiredValue("'cmd' is a rrequired value of a command")

        if isinstance(self.cmd, str):
            self.cmd = shlex.split(self.cmd)

    @staticmethod
    def _attempt_type_convert(value):
        if isinstance(value, str):
            if value.lower() == "true":
                return True
            elif value.lower() == "false":
                return False 
            elif re.compile(r"^[0-9]+\.[0-9]+$").match(value) != None:
                try:
                    v = float(value)
                    return v
                except:
                    return value
            elif re.compile(r"^[0-9]+$").match(value) != None:
                try:
                    v = int(value)
                    return v
                except:
                    return value
        
        return value
            

    def _render_template(self, var, variables):
        if isinstance(var, str) and "{{" in var and "}}" in var:
            template = GLOBAL_ENVIRONMENT.from_string(var)
            rendered = template.render(**variables)

            return self._render_template(rendered, variables)
        
        return self._attempt_type_convert(var)
        
    def _process_conditional(self, item, config, variables):
        CONDITIONAL_TEXT = "Condition {condition} {pf}: {item} ({value}) {sof} {user_specified_item} ({user_specified_value})"
        ALT_CONDITION_TEXT = "Condition {condition} {pf}: {item} ({value}) {sof} {freeform}"

        item_val = self._render_template(item, variables)
        positive = list()
        negative = list()

        if not isinstance(config, dict):
            config = dict(eq=config)

        for key, value in config.items():
            if key == "and":
                p, n = self._process_conditional(item, value, variables)
                
                if len(n) >= 1:
                    negative.append("and condition: " + " or ".join(n))
                else:
                    positive.append("and condition: " + "  or ".join(p))
                
                continue
            elif key == "or":
                p, n = self._process_conditional(item, value, variables)
                
                if len(p) >= 1:
                    positive.append("or condition: " + " or ".join(p))
                else:
                    negative.append("or condition: " + " or ".join(n))
                
                continue

            value_val = self._render_template(value, variables)
            item_type = type(item_val)
            value_type = type(value_val)
            
            if key == "eq":
                if item_type == value_type:
                    if item_val == value_val:
                        text = CONDITIONAL_TEXT.format(condition="eq", pf="passed", item=item, value=item_val, sof="is equal to", user_specified_item=value, user_specified_value=value_val)
                        positive.append(text)
                    else:
                        text = CONDITIONAL_TEXT.format(condition="eq", pf="failed", item=item, value=item_val, sof="is not equal to", user_specified_item=value, user_specified_value=value_val)
                        negative.append(text)
                else:
                    text = CONDITIONAL_TEXT.format(condition="eq_type_check", pf="failed", item=item, value=item_val, sof="does not have the same type as", user_specified_item=value, user_specified_value=value_val)
                    negative.append(text)

            elif key == "neq":
                if type(item_val) == type(value_val):
                    if item_val != value:
                        text = CONDITIONAL_TEXT.format(condition="neq", pf="passed", item=item, value=item_val, sof="is not equal to", user_specified_item=value, user_specified_value=value_val)
                        positive.append(text)
                    else:
                        text = CONDITIONAL_TEXT.format(condition="neq", pf="failed", item=item, value=item_val, sof="is equal to", user_specified_item=value, user_specified_value=value_val)
                        negative.append(text)
                else:
                    text = CONDITIONAL_TEXT.format(condition="neq_type_check", pf="passed", item=item, value=item_val, sof="does not have the same type as", user_specified_item=value, user_specified_value=value_val)
                    positive.append(text)
            elif key == "gt":
                if item_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="gt_type_check", pf="failed", item=item, value=item_val, sof="is not of type integer or float. It is", freeform=str(item_type))
                    negative.append(text)
                    continue
                    
                if value_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="gt_type_check", pf="failed", item=value, value=value_type, sof="is not of type integer or float. It is", freeform=str(value_type))
                    negative.append(text)
                    continue

                if item_val > value_val:
                    text = CONDITIONAL_TEXT.format(condition="gt", pf="passed", item=item, value=item_val, sof="is greater than", user_specified_item=value, user_specified_value=value_val)
                    positive.append(text)
                else:
                    text = CONDITIONAL_TEXT.format(condition="gt", pf="failed", item=item, value=item_val, sof="is not greater than", user_specified_item=value, user_specified_value=value_val)
                    negative.append(text)
            elif key == "ge":
                if item_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="ge_type_check", pf="failed", item=item, value=item_val, sof="is not of type integer or float. It is", freeform=str(item_type))
                    negative.append(text)
                    continue
                    
                if value_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="ge_type_check", pf="failed", item=value, value=value_type, sof="is not of type integer or float. It is", freeform=str(value_type))
                    negative.append(text)
                    continue

                if item_val >= value_val:
                    text = CONDITIONAL_TEXT.format(condition="ge", pf="passed", item=item, value=item_val, sof="is greater than or equal to", user_specified_item=value, user_specified_value=value_val)
                    positive.append(text)
                else:
                    text = CONDITIONAL_TEXT.format(condition="ge", pf="failed", item=item, value=item_val, sof="is not greater than or equal to", user_specified_item=value, user_specified_value=value_val)
                    negative.append(text)
            elif key == "lt":
                if item_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="lt_type_check", pf="failed", item=item, value=item_val, sof="is not of type integer or float. It is", freeform=str(item_type))
                    negative.append(text)
                    continue
                    
                if value_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="lt_type_check", pf="failed", item=value, value=value_type, sof="is not of type integer or float. It is", freeform=str(value_type))
                    negative.append(text)
                    continue

                if item_val < value_val:
                    text = CONDITIONAL_TEXT.format(condition="lt", pf="passed", item=item, value=item_val, sof="is less than", user_specified_item=value, user_specified_value=value_val)
                    positive.append(text)
                else:
                    text = CONDITIONAL_TEXT.format(condition="lt", pf="failed", item=item, value=item_val, sof="is not less than", user_specified_item=value, user_specified_value=value_val)
                    negative.append(text)
            elif key == "le":
                if item_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="le_type_check", pf="failed", item=item, value=item_val, sof="is not of type integer or float. It is", freeform=str(item_type))
                    negative.append(text)
                    continue
                    
                if value_type not in [ int, float ]:
                    text = ALT_CONDITION_TEXT.format(condition="le_type_check", pf="failed", item=value, value=value_type, sof="is not of type integer or float. It is", freeform=str(value_type))
                    negative.append(text)
                    continue

                if item_val >= value_val:
                    text = CONDITIONAL_TEXT.format(condition="le", pf="passed", item=item, value=item_val, sof="is less than or equal to", user_specified_item=value, user_specified_value=value_val)
                    positive.append(text)
                else:
                    text = CONDITIONAL_TEXT.format(condition="le", pf="failed", item=item, value=item_val, sof="is not less than or equal to", user_specified_item=value, user_specified_value=value_val)
                    negative.append(text)

        return positive, negative
        

    def can_run(self, variables):
        if len(self.conditionals) == 0:
            return ( True, [ "No Conditionals" ] )
        else:
            success = []
            failure = []

            for name, conditional in self.conditionals.items():
                if name == "and":
                    for n, c in conditional.items():
                        suc, fail = self._process_conditional(n, c, variables)
                    
                    if len(fail) >= 1:
                        failure.extend(fail)
                    else:
                        success.extend(suc)

                    continue
                if name == "or":
                    for n, c in conditional.items():
                        suc, fail = self._process_conditional(n, c, variables)
                    
                    if len(suc) >= 1:
                        success.extend(suc)
                    else:
                        failure.extend(fail)
                    
                    continue
                    
                
                suc, fail = self._process_conditional(name, conditional, variables)
                
                success.extend(suc)
                failure.extend(fail)
            
            if len(failure) > 0:
                return ( False, failure )
            else:
                return ( True, success )
        
    
    def run_command(self, variables):
        run, messages = self.can_run(variables)
    
        if run:
            if SETTINGS["show_command"] or SETTINGS["show_reason"] or SETTINGS["show_ouput_header"]:
                self.output_to_term("----------------------")

            if SETTINGS["show_command"]:
                self.output_to_term(f"Running {self.name}")
            
            if SETTINGS["show_reason"]:
                if not SETTINGS["show_command"]:
                    self.output_to_term("Reason: {}".format(", ".join(messages)))
                else:
                    self.output_to_term("Reason: {}".format(", ".join(messages)))
            
            for c, pt in enumerate(self.cmd):
                self.cmd[c] = self._render_template(pt, variables)

            if self.mode == "shell":
                command_line = " ".join(self.cmd)
            else:
                command_line = self.cmd
            
            data = []
            if SETTINGS["show_output_header"]:
                if ( not SETTINGS["show_command"] ) and ( not SETTINGS["show_reason"] ):
                    self.output_to_term(f"Output")
                else:
                    self.output_to_term(f"Output")

            if SETTINGS["show_command"] or SETTINGS["show_reason"] or SETTINGS["show_ouput_header"]:
                self.output_to_term("----------------------")
            
            if self.background:
                sp = subprocess.Popen(command_line, shell=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

                if SETTINGS["show_output"]:
                    print("Running In Background")
            else:
                sp = subprocess.Popen(command_line, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)


                with sp as out:
                    data.append(out.stdout.read().decode("utf-8").strip())

                    if SETTINGS["show_output"]:
                        print(data[-1])
                
            if SETTINGS["show_output"]:
                print()
            
            if not self.background:
                return ( sp.returncode, "\n".join(data) )
            else:
                return ( 0, "" )

        else:
            if SETTINGS["show_skip"]:
                if SETTINGS["show_command"] or SETTINGS["show_reason"] or SETTINGS["show_ouput_header"]:
                    self.output_to_term("----------------------")

                if SETTINGS["show_command"]:
                    self.output_to_term(f"Skipping {self.name}")
                
                if SETTINGS["show_command"]:
                    self.output_to_term("Reason: {}".format(", ".join(messages)))
                
                if SETTINGS["show_command"] or SETTINGS["show_reason"] or SETTINGS["show_ouput_header"]:
                    self.output_to_term("----------------------\n")

            return ( 999, "" )

    def output_to_term(self, message):
        if SETTINGS["colour"]:
            print(colored(message, "green"))
            Style.RESET_ALL
        else:
            print(message)
    
    @staticmethod
    def new(name, conf):
        conf["name"] = name
        return Command(**conf)

    def __repr__(self):
        """ String representation of the class """
        items = []

        for k, v in self.__dict__.items():
            if "_" != k[0]:
                if "pass" in k:
                    v = '*' * len(v)
                
                if isinstance(v, str):
                    items.append(f"{k}='{v}'")
                else:
                    items.append(f"{k}={v}")

        items = ', '.join(items)

        return f"{self.__class__.__name__}({items})"