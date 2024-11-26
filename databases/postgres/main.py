# On ubuntu run 'sudo apt-get install libpq-dev python3-dev' to install needed dependencies
import psycopg2
from psycopg2 import sql
from psycopg2.extensions import connection, cursor


# 3 Insert data
# 4. Upsert Data
# 5 Delete Data
# 6 dumps

db_config = {
    "dbname": "test",
    "user": "admin",
    "password": "supersecret",
    "host": "localhost",
    "port": 3333
}


def get_cursor():
    conn: connection = psycopg2.connect(**db_config)
    cursor = conn.cursor()
    return cursor


def main():
    cursor = get_cursor()
    #  SELECT all flats
    cursor.execute("SELECT * FROM flats;")
    all_flats = cursor.fetchall()
    print(f"Fetched {len(all_flats)} flats")

    cursor.close()


main()
