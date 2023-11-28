# EZQL API

EZQL (Easy Zettelkasten Query Language) is designed to be a basic API to interact with the ezql database wherein tables and strong references are abandoned in favor of tags and indirect relations. Such a database is useful for massive data management and sorting, allowing for storage optimization by reducing empty columns and instead only inputting relevant data. 

The database is structured and designed in such a way that querying through data does not necessitate the loading of said data. This is infinitely scalable, allowing for extreme precision or extreme speed, with a balance being comfortable at both. This API will be required in order to properly interface with the database, and front-end programs or server OS systems may directly implement the API in order to facilitate a user-friendly or graphically visible experience. Any data can be stored, in the form of any file; text or numerical data can also be stored directly on an entry. For large data or separate files, the address pointer will be stored in the entry instead of the data, allowing for blazing-fast data searching and query responses, with only the final file load taking a normal amount of time to execute. 

## TO DO

* Command parsing and complex multi-operational command entries.
* Commands to actually create and write to the files with null checks and overwrite security in place.
* Creating the CLI that will implement this API for quick, easy install and use of the EZQL library.
