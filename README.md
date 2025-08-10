## Usage

### Single equation
```bash
cargo run -- -q "y=x^2"
```

### Multiple equations
```bash
cargo run -- -q "y=sin(x)" -q "y=cos(x)"
```

### Custom x/y range
```bash
cargo run -- -q "y=tan(x)" --xmin -3.14 --xmax 3.14 --ymin -2 --ymax 2
```

### Example with `pi` and `log`
```bash
cargo run -- -q "y=sin(pi*x)" -q "y=log(x,10)" --xmin -1 --xmax 10
```

## Command-Line Options

| Option        | Description |
|---------------|-------------|
| `-q` `--queries` | Equation(s) to plot, in the form `y=...`. Can be used multiple times. |
| `--xmin`      | Minimum value for the x-axis. |
| `--xmax`      | Maximum value for the x-axis. |
| `--ymin`      | Minimum value for the y-axis. |
| `--ymax`      | Maximum value for the y-axis. |
| `-h`, `--help` | Show help. |
| `-V`, `--version` | Show version. |
