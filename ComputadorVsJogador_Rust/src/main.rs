extern crate rand;

use std::io;
use rand::Rng;

fn main() {
  let mut vetorSorteio: Vec<i32> = Vec::new();
  let mut indiceSorteio = 0;
  let mut max;
  let mut tentativasComputador;
  let mut tentativasJogador;

  println!("Escolha o limite maximo dos valores a serem sorteados (Esse valor deve ser maior que 20 e menor que 350)");
  let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let mut numero:i32 = leitura.trim().parse()
  .expect("Failed to convert");
  max = numero;
  while max <= 20 || max > 350 {
    println!("Valor menor que 20 ou maior que 350. Escreva novamente\n");
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let mut numero:i32 = leitura.trim().parse()
  .expect("Failed to convert");
    max = numero;
  }

  println!("Gerando 20 valores aleatorios de 1 até {}:\n\n", max);
  let mut vectSort = &mut vetorSorteio;
  gerarNumeros(vectSort, max);
  arrumaVetor(vectSort);
  
  print!("Vetor Sorteio: ");
  let mut i = 0;
  let mut i_usize: usize = i as usize;
  while i_usize < 20 {
    print!("{} ", vetorSorteio[i_usize]);
    i_usize += 1;
  }
  println!("");
  
  let indiceSorteio = rand::thread_rng().gen_range(0, 19);

  vectSort = &mut vetorSorteio;
  println!("Sua vez: ");
  tentativasJogador = gamePessoa(max, vectSort, indiceSorteio);
  println!("");

  println!("Vez do computador: ");
  tentativasComputador = buscaBinaria(indiceSorteio, vectSort);
  println!("");
  
  if tentativasJogador > tentativasComputador {
    println!("Computador ganhou de {tentativasComputador} tentativas contra {tentativasJogador} tentativas");
  }else if tentativasJogador < tentativasComputador {
    println!("Jogador ganhou de {tentativasJogador} tentativas contra {tentativasComputador} tentativas");
  }else{
    println!("Jogo Empatou");
  }
}

fn gerarNumeros(vetorSorteio: &mut Vec<i32>, max: i32){
  let mut vetorValores: Vec<i32> = Vec::new();

  let mut i: i32 = 1;
  while i <= max {
    vetorValores.push(i);
    i += 1;
  }
  
  i = 0;
  let mut i_usize: usize = i as usize;
  while i < 20{
    let indice = rand::thread_rng().gen_range(1, max-(i));
    let indice_usize: usize = indice as usize;
    vetorSorteio.push(vetorValores[indice_usize]);

    vetorValores.remove(indice_usize);
    
    i += 1;
    i_usize += 1;
  }
}

fn arrumaVetor(C: &mut Vec<i32>){
  let mut maior = 0;
  let mut j = 0;
  let mut i = 0;
  while i < 20 {
    j = i+1;
    while j < 20 {
      if C[i] > C[j] {
        maior = C[j];
        C[j] = C[i];
        C[i] = maior;
      }
      j += 1;
    }
    i += 1;
  }
}

fn gamePessoa(max: i32, vetorSorteio: &mut Vec<i32>, indiceSorteio: i32) -> i32{
  let indiceSorteio_usize: usize  = indiceSorteio as usize;
  let mut cont: i32 = 0;
  let mut acertoPessoa = 0;
  let mut tentativa;
  while acertoPessoa < 1 {
    loop {
      cont += 1;
      println!("Tentativa {cont}"); 
      println!("Digite um numero entre 1 e {max}");
      let mut leitura = String::new();

      io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

      let numero:i32 = leitura.trim().parse()
      .expect("Failed to convert");
      tentativa = numero;
      if tentativa < 1 || tentativa > max {
        println!("Numero fora do limite. Tente novamente.");
      }else{
        break;
      }
    }
    if tentativa == vetorSorteio[indiceSorteio_usize] {
      println!("Voce Acertou na {cont} tentativa!");
      break;
    }else{
      if tentativa < vetorSorteio[indiceSorteio_usize] {
        println!("Voce errou! Muito baixo o valor.");
        println!("");
      }else{
        println!("Voce errou! Muito alto o valor.");
        println!("");
      }
    }
  }
    
  return cont;
}

fn buscaBinaria(indiceSorteio:i32, vetorSorteio: &mut Vec<i32>) -> i32{
  let mut acertoComputador = 0;
  let mut cont = 1;
  let mut indiceMax = 19;
  let mut organizaBin = 10;
  let mut indiceBin = indiceMax - organizaBin;
  let mut indiceBin_usize = indiceBin as usize;
  
  println!("Tentativa {cont}: {}", vetorSorteio[indiceBin_usize]);
      
  while acertoComputador < 1 {
    if indiceBin == indiceSorteio {
      acertoComputador = 1;
      break;
    }else{
      if indiceBin > indiceSorteio {
        if organizaBin != 1 {
          organizaBin = organizaBin/2;
        }
        indiceBin -= organizaBin;
        cont += 1;
        indiceBin_usize = indiceBin as usize;
        println!("Tentativa {cont}: {}", vetorSorteio[indiceBin_usize]);
      }else{
        if indiceBin == 14{
          indiceBin = 17;
          cont += 1;
          println!("Tentativa {cont}: {}", vetorSorteio[17]);
        }else{
          if organizaBin != 1 {
            organizaBin = organizaBin/2;
          }
          indiceBin += organizaBin;
          cont += 1;
          indiceBin_usize = indiceBin as usize;
          println!("Tentativa {cont}: {}", vetorSorteio[indiceBin_usize]);
        }
      }
    }
  }
  
  return cont;
}