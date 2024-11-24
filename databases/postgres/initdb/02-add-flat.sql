-- LOAD data into tables
INSERT INTO flats (flat_id, district, street, rooms, floors_total, floor, price, area, short_description, updated_at, created_at, picture, is_filtered) 
VALUES ('id_1', 'Centre', 'Tomsona', 2, 5, 1, 100000,  50.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_2', 'Purvciems', 'Marcienas', 3, 5, 3, 150000, 70.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_3', 'Plyavnieki', 'Lubanas', 1, 9, 3, 80000, 30.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_4', 'Imanta', 'Zolitudes', 4, 5, 4, 200000,  90.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, FALSE),
('id_5', 'Imanta', 'Brivibas', 5, 5, 3, 250000, 110.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_6', 'Mazhapark', 'Gdanskas', 2, 5, 3, 100000,  50.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_7', 'Centre', 'Vesetas', 3, 9, 7, 150000, 70.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_8', 'Centre', 'Tallinas', 1, 5, 3, 80000, 30.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_9', 'Purvciems', 'Marcienas', 3, 5, 3, 150000, 70.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_10', 'Mazhapark', 'Gdanskas', 2, 5, 3, 100000,  50.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE),
('id_11', 'Imanta', 'Brivibas', 5, 5, 3, 250000, 110.00, 'Nice flat in the center of the city', NOW(), NOW(), NULL, TRUE);

INSERT INTO flat_updates (flat_1_id, flat_2_id)
VALUES ('id_2', 'id_9'),
('id_5', 'id_11'),
('id_6', 'id_10')

