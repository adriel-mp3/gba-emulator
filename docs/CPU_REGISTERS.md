# Registradores da CPU do GBA

O Game Boy Advance usa uma CPU **ARM7TDMI**, com registradores de 32 bits. Eles armazenam valores, endereços, o estado da execução e informações necessárias para chamadas de função e interrupções.

## Registradores principais

| Registrador | Nome | Função |
|---|---|---|
| `R0`–`R12` | Gerais | Usados para cálculos, valores temporários e passagem de argumentos. Não possuem uma função fixa. |
| `R13` (`SP`) | Stack Pointer | Aponta para o topo atual da pilha, usada para guardar dados temporários e retornos. |
| `R14` (`LR`) | Link Register | Guarda o endereço de retorno quando uma função ou rotina é chamada. |
| `R15` (`PC`) | Program Counter | Contém o endereço da instrução que está sendo executada ou buscada. |

O ARM7TDMI possui versões separadas (*banked registers*) de alguns registradores em modos de exceção. Assim, uma interrupção pode usar seu próprio `SP` e `LR` sem sobrescrever os valores do programa interrompido.

## CPSR

O **CPSR** (*Current Program Status Register*) guarda o estado atual da CPU. Ele contém as flags, o modo de execução, o estado ARM/Thumb e o controle de interrupções.

### Flags de condição

| Flag | Nome | Significado |
|---|---|---|
| `N` | Negative | É definida quando o resultado de uma operação é negativo. Normalmente copia o bit mais significativo do resultado. |
| `Z` | Zero | É definida quando o resultado da operação é zero. |
| `C` | Carry | Indica carry em uma soma ou ausência de borrow em uma subtração; também pode guardar bits deslocados. |
| `V` | Overflow | Indica overflow aritmético com números com sinal, quando o resultado não cabe em 32 bits. |

As instruções condicionais consultam essas flags. Por exemplo, `BEQ` executa o salto quando `Z = 1`, enquanto `BNE` executa quando `Z = 0`.

## Outros bits importantes do CPSR

- **T**: seleciona o estado Thumb (`1`) ou ARM (`0`).
- **I**: mascara interrupções normais quando está definida.
- **F**: mascara interrupções rápidas (FIQ) quando está definida.
- **Modo**: indica o modo atual da CPU, como usuário, supervisor ou IRQ.

## SPSR

O **SPSR** (*Saved Program Status Register*) existe nos modos de exceção. Ao entrar em uma exceção, ele guarda uma cópia do CPSR anterior. Ao retornar, a CPU pode restaurar o estado original do programa.

## Resumo

```text
R0–R12 = valores e cálculos temporários
R13    = SP: pilha
R14    = LR: endereço de retorno
R15    = PC: fluxo de execução
CPSR   = estado atual e flags
SPSR   = estado salvo durante exceções
```
