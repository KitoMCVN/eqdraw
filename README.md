**eqdraw** is a simple yet powerful command-line tool that lets you visualize mathematical functions directly in your terminal using Unicode characters.

![eqdraw screenshot](https://cdn.discordapp.com/attachments/1398189355528487044/1404306116661477506/Screenshot_2025-08-11-10-28-53-822_com.android.chrome.png?ex=689ab57c&is=689963fc&hm=244c195fd1ef011d184afcf5b96f056c81aa8b6c430093b55a49a6ee3513e709&)

## ⚙️ Install

You'll need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system. Then, run the following command:

```bash
cargo install eqdraw
```

## 🚀 Quick Usage

Using `eqdraw` is straightforward. Here are a few examples:

**1. Plot a single equation:**

```bash
eqdraw -q "y=x^2"
```

**2. Plot multiple equations on the same graph:**

```bash
eqdraw -q "y=sin(x)" -q "y=cos(x)"
```

**3. Customize the view range:**

Focus on a specific part of the graph by defining the min/max values for the axes.

```bash
eqdraw -q "y=tan(x)" --xmin -3.14 --xmax 3.14 --ymin -2 --ymax 2
```

## 📝 Equation Syntax

Equations must follow the format `y=<expression_in_x>`.

### Operators

| Operator | Description |
| :--- | :--- |
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `^` | Power |

### Constants

| Name | Value |
| :--- | :--- |
| `pi` | The mathematical constant Pi (π ≈ 3.14159) |
| `e` | Euler's number (e ≈ 2.71828) |

### Functions

| Function | Description |
| :--- | :--- |
| `sqrt(x)` | The square root of x |
| `exp(x)` | e raised to the power of x |
| `pow(b, e)`| b raised to the power of e |
| `root(x, n)`| The nth root of x |
| `abs(x)` | The absolute value of x |
| `sin(x)` | The sine of x (x in radians) |
| `cos(x)` | The cosine of x (x in radians) |
| `tan(x)` | The tangent of x (x in radians) |
| `cot(x)` | The cotangent of x (x in radians) |
| `asin(x)` | The arcsine of x |
| `acos(x)` | The arccosine of x |
| `atan(x)` | The arctangent of x |
| `sinh(x)` | The hyperbolic sine of x |
| `cosh(x)` | The hyperbolic cosine of x |
| `tanh(x)` | The hyperbolic tangent of x |
| `ln(x)` | The natural logarithm of x (base e) |
| `log(x)` | The base-10 logarithm of x |
| `logb(x, b)`| The logarithm of x with base b |
| `floor(x)`| Rounds x down to the nearest integer |
| `ceil(x)` | Rounds x up to the nearest integer |
| `round(x)`| Rounds x to the nearest integer |

## 🔧 Command-line Options

| Option | Alias | Description |
| :--- | :--- | :--- |
| `--query <QUERY>` | `-q` | The equation to plot. Can be used multiple times. |
| `--xmin <XMIN>` | | Minimum value for the x-axis. |
| `--xmax <XMAX>` | | Maximum value for the x-axis. |
| `--ymin <YMIN>` | | Minimum value for the y-axis. |
| `--ymax <YMAX>` | | Maximum value for the y-axis. |
| `--help` | `-h` | Show the help message. |
| `--version` | `-V` | Show the version information. |

## 💡 Inspiration

This project was inspired by the fantastic Python library [plotille](https://github.com/tammoippen/plotille) by Tammo Ippen.

## 🤝 Contributing

Contributions are welcome! If you have an idea for an improvement or a bug fix, please open an Issue or a Pull Request on the repository.
