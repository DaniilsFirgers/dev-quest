# import psycopg

import requests


def main():
    img_data = requests.get(
        'https://i.ss.com/gallery/7/1278/319301/63860018.th2.jpg').content
    with open('image_name.jpg', 'wb') as handler:
        handler.write(img_data)


main()
