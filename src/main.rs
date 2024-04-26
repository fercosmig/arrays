fn main()
{
    {
        // usando o método simples e padrão para acessar os itens do array
        let numeros_inteiros = [10, 20, 30, 40, 50];

        println!("A0:{}", numeros_inteiros[0]);
        println!("A1:{}", numeros_inteiros[1]);
        println!("A2:{}", numeros_inteiros[2]);
        println!("A3:{}", numeros_inteiros[3]);
        println!("A4:{}", numeros_inteiros[4]);
    }

    {
        // usando um for comum para acessar os itens do array
        let numeros_inteiros = [60, 70, 80, 90, 100];
        for i in 0..numeros_inteiros.len()
        {
            println!("B{}:{}", i, numeros_inteiros[i]);
        }
    }

    {
        // usando o "iter" para acessar os métodos do array
        let numeros_inteiros = [110, 120, 130, 140, 150];
        for n in numeros_inteiros.iter()
        {
            println!("D:{}", n);
        }
    }

    {
        // declarando o array com tipo e quantidade de posições
        let numeros_inteiros: [i32; 5] = [160, 170, 180, 190, 200];
        for n in numeros_inteiros.iter()
        {
            println!("E:{}", n);
        }
    }

    {
        // declarando o array com um valor e a quantidade de vezes que o mesmo vai se repetir
        let numeros_inteiros = [2; 5];
        for n in numeros_inteiros.iter()
        {
            println!("E:{}", n);
        }
    }
}
