# Forge

Data file generation cli tool

- csv / tsv / txt
- xml
- json

## Usage
```
forge.exe csv --path C:\documents\file.csv --records 100 --delim "," --columns "index:index, id:uuid, name:full_name, email:email""
```
```
forge.exe csv --path C:\documents\file.tsv --records 100 --columns "index:index, id:uuid, name:full_name, email:email""
```
```
forge.exe xml --path C:\documents\file.csv --records 100 --columns "index:index, id:uuid, name:full_name, email:email""
```
```
forge.exe json --path C:\documents\file.csv --records 100 --columns "index:index, id:uuid, name:full_name, email:email""
```
