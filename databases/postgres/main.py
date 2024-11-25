# import psycopg

import requests

# 1. create indexes on start up
# 2. create indexes while running
# 3 Insert data
# 4. Upsert Data


def main():
    img_data = requests.get(
        'https://i.ss.com/gallery/7/1278/319301/63860018.th2.jpg').content
    with open('image_name.jpg', 'wb') as handler:
        handler.write(img_data)


main()
