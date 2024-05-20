# CSV-grep

This program allows you to manipulate and display data from a CSV file using command-line options. Below are the available options:

### Options

- **-f, --first `<N>`** : Display the first N lines of the file. By default, N is 10.

  Example: `-f 5` displays the first 5 lines.

- **-l, --last `<N>`** : Display the last N lines of the file. By default, N is 10.

  Example: `-l 5` displays the last 5 lines.

- **-n, --column-name `<name>`** : Display the column specified by its name.

  Example: `-n column_A` displays the column named "column_A".

- **-s, --sep `<sep>`** : Define the column separator in the CSV file. By default, the separator is a comma (`,`).

  Example: `-s ";"` to use a semicolon as the separator.

- **-i, --column-index `<index>`** : Display the column specified by its index (starting from 0).

  Example: `-i 4` displays the fourth column (1-indexed).

- **--pretty** : Pretty print the dataframe.

  Usage: `--pretty` to enable pretty printing.

- **--no-header** : Indicate that the file does not contain a header row.

  Usage: `--no-header` if the CSV file does not have a header.

- **-h, --help** : Print this help menu and exit.

  Usage: `-h` or `--help` to display this help message.

### Command Examples

- Display the first 10 lines of the file `data.csv`:

  ```sh
  csvgrep.exe -f 10 wines.csv
  ```

  ```csv
  "Wine","Alcohol","Malic.acid","Ash","Acl","Mg","Phenols","Flavanoids","Nonflavanoid.phenols","Proanth","Color.int","Hue","OD","Proline"
  "1","14.23","1.71","2.43","15.6","127","2.8","3.06",".28","2.29","5.64","1.04","3.92","1065"
  "1","13.2","1.78","2.14","11.2","100","2.65","2.76",".26","1.28","4.38","1.05","3.4","1050"
  "1","13.16","2.36","2.67","18.6","101","2.8","3.24",".3","2.81","5.68","1.03","3.17","1185"
  ...
  ```

- Pretty display the last 3 lines of the file `data.csv`:

  ```sh
  csvgrep.exe -l 3 --pretty wines.csv
  ```

  ```csv
  | Wine | Alcohol | Malic.acid | Ash  | Acl  | Mg  | Phenols | Flavanoids | Nonflavanoid.phenols | Proanth | Color.int | Hue | OD   | Proline |
  +------+---------+------------+------+------+-----+---------+------------+----------------------+---------+-----------+-----+------+---------+
  | 3    | 13.27   | 4.28       | 2.26 | 20   | 120 | 1.59    | .69        | .43                  | 1.35    | 10.2      | .59 | 1.56 | 835     |
  | 3    | 13.17   | 2.59       | 2.37 | 20   | 120 | 1.65    | .68        | .53                  | 1.46    | 9.3       | .6  | 1.62 | 840     |
  | 3    | 14.13   | 4.1        | 2.74 | 24.5 | 96  | 2.05    | .76        | .56                  | 1.35    | 9.2       | .61 | 1.6  | 560     |
  ```

- Display the column named "Alcohol" and pretty print the output:

  ```sh
  csvgrep.exe -n Alcohol --pretty data.csv
  ```

  ```
  | Alcohol |
  +---------+
  | 14.23   |
  | 13.2    |
  | ...     |
  | 14.13   |
  ```

- Display the third column of a file without a header:

  ```
  csvgrep.exe -i 3 --no-header data.csv
  ```

  ```
  "1.71"
  "1.78"
  ...
  "4.1"
  ```
