#include <vector>
#include <thread>
#include <iostream>

class Numero {
    int* numero;
    public:
        Numero(int numero) {
            this->numero = new int(numero);
        }

        /*-----------------------------*/
        /*Con el constructor por move  */
        /*obtenemos un segfault. Sin   */
        /*obtenemos un double free     */
        /*-----------------------------*/

        Numero(Numero&& otro) {
            this->numero = otro.numero;
            otro.numero = nullptr;
        }

        void imprimir() {
            std::cout << "EL numero es: " << *this->numero << std::endl;
        }

        ~Numero() {
            delete this->numero;
        }
};

void move_from_to(std::vector<Numero>& from, std::vector<Numero>& to) {
    while (!from.empty()) {
        Numero numero = std::move(from.back());
        from.pop_back();
        to.push_back(std::move(numero));
    }
}

int main() {
    std::vector<Numero> to;
    std::vector<Numero> from;

    from.emplace_back(0);
    from.emplace_back(1);
    from.emplace_back(2);
    from.emplace_back(3);
    from.emplace_back(4);
    from.emplace_back(5);
    from.emplace_back(6);
    from.emplace_back(7);
    from.emplace_back(8);
    from.emplace_back(9);

    std::thread th_a(move_from_to, ref(from), ref(to));
    std::thread th_b(move_from_to, ref(from), ref(to));
    move_from_to(from, to);

    th_a.join();
    th_b.join();

    for (auto& number : to)
        number.imprimir();

    return 0;
}