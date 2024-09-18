// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
use std::borrow::Cow;

use cdk_from_cfn::code::{CodeBuffer, IndentOptions};

const INDENT: Cow<'static, str> = Cow::Borrowed("    ");

pub trait CdkAppCodeWriter {
    fn app_file(&self, code: &CodeBuffer, cdk_stack_classname: &str);
}

pub struct Typescript {}

impl CdkAppCodeWriter for Typescript {
    fn app_file(&self, code: &CodeBuffer, cdk_stack_classname: &str) {
        code.line("// auto-generated! a human should update this!");
        code.line("import * as cdk from \"aws-cdk-lib\";");
        code.line(format!(
            "import {{ {} }} from \"./stack\";",
            cdk_stack_classname
        ));
        let app = code.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("const app = new cdk.App({".into()),
            trailing: Some("});".into()),
            trailing_newline: true,
        });
        let app_props = app.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("defaultStackSynthesizer: new cdk.DefaultStackSynthesizer({".into()),
            trailing: Some("}),".into()),
            trailing_newline: true,
        });
        app_props.line("generateBootstrapVersionRule: false,");
        code.line(format!("new {}(app, \"Stack\");", cdk_stack_classname));
        code.line("app.synth();");
    }
}

pub struct Python {}

impl CdkAppCodeWriter for Python {
    fn app_file(&self, code: &CodeBuffer, cdk_stack_classname: &str) {
        code.line("# autogenerated");
        code.line("import aws_cdk as cdk");
        code.line(format!("from stack import {cdk_stack_classname}"));
        let app = code.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("app = cdk.App(".into()),
            trailing: None,
            trailing_newline: true,
        });
        let app_props = app.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("default_stack_synthesizer=cdk.DefaultStackSynthesizer(".into()),
            trailing: None,
            trailing_newline: false,
        });
        app_props.line("generate_bootstrap_version_rule=False");
        app_props.line(")");
        app.line(")");
        // make generic
        code.line(format!("{cdk_stack_classname}(app, 'Stack')"));
        code.line("app.synth()");
    }
}

pub struct Java {}

impl CdkAppCodeWriter for Java {
    fn app_file(&self, code: &CodeBuffer, cdk_stack_classname: &str) {
        println!("Writing java app file");
        code.line("//auto-generated");
        code.line("package com.myorg;");
        code.line("import software.amazon.awscdk.App;");
        code.line("import software.amazon.awscdk.AppProps;");
        code.line("import software.amazon.awscdk.DefaultStackSynthesizer;");
        code.line("import software.amazon.awscdk.StackProps;");
        let main_class = code.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("public class MyApp {".into()),
            trailing: Some("}".into()),
            trailing_newline: true,
        });

        let main_function = main_class.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("public static void main(final String[] args) {".into()),
            trailing: Some("}".into()),
            trailing_newline: true,
        });
        let app_constructor = main_function.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("App app = new App(AppProps.builder()".into()),
            trailing: None,
            trailing_newline: true,
        });
        let stack_synthesizer_props = app_constructor.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some(
                ".defaultStackSynthesizer(DefaultStackSynthesizer.Builder.create()".into(),
            ),
            trailing: None,
            trailing_newline: false,
        });
        stack_synthesizer_props.line(".generateBootstrapVersionRule(false)");
        stack_synthesizer_props.line(".build())");
        app_constructor.line(".build());");

        let stack_props = app_constructor.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some(
                format!(
                    "new {}(app, \"Stack\", StackProps.builder()",
                    cdk_stack_classname
                )
                .into(),
            ),
            trailing: None,
            trailing_newline: false,
        });
        stack_props.line(".build());");
        app_constructor.line("app.synth();");
    }
}

pub struct CSharp {}

impl CdkAppCodeWriter for CSharp {
    fn app_file(&self, code: &CodeBuffer, cdk_stack_classname: &str) {
        println!("Generating Program.cs file");
        code.line("//Auto-generated");
        code.line("using Amazon.CDK;");
        code.line("sealed class Program");
        let main_class = code.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("{".into()),
            trailing: Some("}".into()),
            trailing_newline: true,
        });
        main_class.line("public static void Main(string[] args)");
        let main_function = main_class.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("{".into()),
            trailing: Some("}".into()),
            trailing_newline: true,
        });
        main_function.line("var app = new App(new AppProps");
        let app_constructor = main_function.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("{".into()),
            trailing: Some("});".into()),
            trailing_newline: true,
        });
        app_constructor.line("DefaultStackSynthesizer = new DefaultStackSynthesizer(new DefaultStackSynthesizerProps");
        let stack_synthesizer_props = app_constructor.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("{".into()),
            trailing: Some("}),".into()),
            trailing_newline: true,
        });
        stack_synthesizer_props.line("GenerateBootstrapVersionRule = false,");

        main_function.line(format!(
            "new {}.{}(app, \"Stack\");",
            cdk_stack_classname, cdk_stack_classname
        ));
        main_function.line("app.Synth();");
    }
}

pub struct Golang {}

impl CdkAppCodeWriter for Golang {
    fn app_file(&self, code: &CodeBuffer, cdk_stack_classname: &str) {
        println!("generating go app file");
        code.line("// auto-generated");
        code.line("package main");
        let imports = code.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("import (".into()),
            trailing: Some(")".into()),
            trailing_newline: true,
        });
        imports.line("\"github.com/aws/aws-cdk-go/awscdk/v2\"");
        imports.line("\"github.com/aws/jsii-runtime-go\"");

        let main_function = code.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("func main() {".into()),
            trailing: Some("}".into()),
            trailing_newline: true,
        });
        main_function.line("defer jsii.Close()");
        let app_constructor = main_function.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("app := awscdk.NewApp(&awscdk.AppProps{".into()),
            trailing: Some("})".into()),
            trailing_newline: true,
        });
        let stack_synthesizer_props = app_constructor.indent_with_options(IndentOptions {
            indent: INDENT,
            leading: Some("DefaultStackSynthesizer: awscdk.NewDefaultStackSynthesizer(&awscdk.DefaultStackSynthesizerProps{".into()),
            trailing: Some("}),".into()), 
            trailing_newline: true,
        });
        stack_synthesizer_props.line("GenerateBootstrapVersionRule: jsii.Bool(false),");

        main_function.line(format!("New{cdk_stack_classname}(app, \"Stack\", nil)"));
        main_function.line("app.Synth(nil)");
    }
}
