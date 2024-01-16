# autogenerated
import aws_cdk as cdk
from stack import JsonPropsStack
app = cdk.App(
    default_stack_synthesizer=cdk.DefaultStackSynthesizer(
        generate_bootstrap_version_rule=False
        )
    )

JsonPropsStack(app, 'Stack')
app.synth()
