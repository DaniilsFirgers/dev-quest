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

    # SELECT with condition
    cursor.execute("SELECT * FROM flats WHERE price < 100000;")
    cheap_flats = cursor.fetchall()
    print(f"Fetched with condition {len(cheap_flats)} flats")

    # SELECT with condition, but it return 4 columns
    cursor.execute(
        "SELECT flat_id, price, area, rooms FROM flats WHERE rooms = 3;")
    three_room_flats = cursor.fetchall()
    print(f"Using WHERE:  {three_room_flats}")

    # SELECT with condition, but return 4 columns sorted by price
    cursor.execute(
        "SELECT flat_id, price, area, rooms FROM flats WHERE rooms > 2 ORDER BY price DESC;")
    three_room_flats_sorted = cursor.fetchall()
    print(f"Ordered flats:  {three_room_flats_sorted}")

    # SELECT with condition, but return 4 columns sorted by price and limit is 2 (TOP 2 by room)
    cursor.execute(
        "SELECT flat_id, price, area, rooms FROM flats WHERE rooms > 2 ORDER BY price DESC LIMIT 2;"
    )
    top_two_three_room_flats = cursor.fetchall()
    print(f"Top two flats:  {top_two_three_room_flats}")

    # Some queries with SQL module
    cursor.close()


main()
