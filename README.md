# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber 
is defined as an interface. Explain based on your understanding of Observer design patterns,
do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model
struct is enough?

Menurut saya tidak diperlukan untuk menggunakan interface atau trait pada project 
Rust ini. Hal ini dikarenakan Subscriber dalam BambangShop diwakili satu class dan
memiliki perilaku yang sama. Semua instance Subscriber yang mempunyai perilaku serupa 
dan tidak perlu untuk memisahkan perilaku ke dalam trait terpisah dalam projek ini

2. id in Program and url in Subscriber is intended to be unique. Explain based on your
   understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently
   use is necessary for this case?

DashMap disini bekerja mirip dengan HashMap pada bahasa pemrograman Java. DashMap
lebih baik digunakan karena kita bisa menggunakan key yang unik untuk menyimpan item
yang ada. Selain itu, DashMap juga memungkinkan kita untuk mengecek key duplikat dengan lebih cepat
dibandingkan menggunakan Vec. Jika kita tetap menggunakan Vec, kita harus melakukn iterasi untuk
semua item yang ada untuk mengecek duplikat sehingga memerlukan waktu yang lebih banyak

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a
   thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we
   used the DashMap external library for thread safe HashMap. Explain based on your
   understanding of design patterns, do we still need DashMap or we can implement Singleton
   pattern instead?

Menuru saya, penggunaan DashMap diperlukan untuk mengelola state secara thread-safe. Singleton dapat digunakan
bila kita membutuhkan kontrol lebih besar terhadap pembuatan instance dan akses ke data
yang di-share. Biasanya DashMap lebih disarankan karena singleton dapat membuat tight coupling
sehingga sulit untuk membuat unit testing. Namun, singleton bisa digunakan bila membutuhkan kontrol yang spesifik


#### Reflection Publisher-2

1. Kita melakukan separation of concern dengan memisahkan 
Service dan Repository. Hal ini dilakukan untuk mengikuti SRP
serta meningkatkan maintainability dan flexibility dari program kita karena
komponen yang lebih terfokus dan tidak terlalu bergantung satu sama lain

2. Jika hanya menggunakan Model, tanpa pemisahan service dan controller,
program akan menjadi jauh lebih kompleks serta rawan untuk terjadi duplikasi kode dan tight coupling.
Model akan meng-handle tidak hanya bagian data, namun juga logic yang ada dalam program.
Hal ini membuat program kita lebih sulit untuk di-maintain sehingga dapat memakan banyak
biaya selama program tersebut tetap berjalan
3. Sudah, saya sudah mengetahui Postman sejak mengambil mata kuliah PBP.
Postman berguna untuk API testing seperti mengecek apakah endpoint yang saya buat
menerima data dengan baik

#### Reflection Publisher-3

1. Kita menggunakan push model karena bagian NotificationService
melakukan push data ke subscriber saat da subscriber yang melakukan add, delete, atau publish
product
2. Jika menggunakan metode lain seperti pull, subscriber/observer lain bisa saja tidak mendapatkan notifikasi
karena subscriberlah yang melakukan pull data dari publisher. Jadi subscriber harus aktif untuk meminta pull data dari publisher.
Walaupun sekarang dapat mengambil data yang relevan saja sesuai kebutuhannya, tetapi sistem notifikasi
tidak akan berjalan secara otomatis setiap ada proses update
3. Jika tidak menggunakan multi-threading, performa aplikasi akan menurun karena
prose yang terjadi secara sequensial atau satu per satu secara berurutan. Proses ini menyebabkan
request mengantri dan menyebabkan aplikasi menjadi lambat

