#!/usr/bin/env python3
import click
import yaml
from pathlib import Path

@click.group()
def main():
    pass

@main.command()
@click.argument('spec_file')
@click.option('--output', '-o', default='.team/tools/generated')
def generate(spec_file, output):
    print(f"Generating from {spec_file} to {output}")
    with open(spec_file) as f:
        spec = yaml.safe_load(f)
    
    output_path = Path(output)
    output_path.mkdir(exist_ok=True)
    
    for tool in spec.get('tools', []):
        name = tool['name']
        with open(output_path / f"{name}.py", 'w') as f:
            f.write(f"#!/usr/bin/env python3\n")
            f.write(f"# {name}\n")
            f.write(f"def {name.replace('-', '_')}():\n")
            f.write('    print(f"TODO: implement {name}")\n')
        print(f"Generated {name}.py")

if __name__ == '__main__':
    main()
