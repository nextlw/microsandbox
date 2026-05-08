[English](./CONTRIBUTING.md) · Português (Brasil)

# Contribuindo com o Microsandbox

Olá! Quer você já seja uma pessoa desenvolvedora experiente ou esteja começando agora, estamos muito felizes por seu interesse em contribuir com o Microsandbox. Esta comunidade existe graças a pessoas como você!

## Boas-vindas, novos contribuidores!

Nunca contribuiu para software livre antes? Sem problema! Todo mundo já passou por isso. Marcamos issues com [good first issues][good-first-issues] para ajudar você a achar um ponto de partida confortável. Lembre-se: todo especialista já foi iniciante!

## Precisa de ajuda?

Estamos à disposição! Escreva para [development@superrad.company][support-email] com qualquer dúvida. Este guia foi feito para ajudar você a começar; se parecer detalhado demais, saiba que a ideia é ser completo, não intimidador!

## Código de conduta

Nossa comunidade valoriza gentileza e inclusão. Reserve um momento para ler o nosso [código de conduta](./CODE_OF_CONDUCT.pt-BR.md) e nos ajudar a manter um ambiente acolhedor para todas as pessoas.

## Configuração para desenvolvimento

Para um guia completo sobre como compilar, testar e publicar o microsandbox, veja o **[DEVELOPMENT.md](./DEVELOPMENT.pt-BR.md)**.

## Como contribuir

Há muitas formas de contribuir com o Microsandbox, e agradecemos todas! Veja por onde começar:

### Ideias e feedback

#### Issues

Encontrou um bug ou tem uma ideia de funcionalidade? Queremos saber!

Antes de abrir uma issue nova, busque nas existentes para evitar duplicatas. Depois use o botão de issues, escolha um modelo e compartilhe sua ideia. Não se preocupe se não conseguir preencher o modelo inteiro — envie o que puder; você sempre pode atualizar depois.

#### Discussões

Para assuntos que não são exatamente bug ou pedido de funcionalidade, abra uma [discussão no GitHub][gh-discussions]. Bons temas incluem:

- Comunicados da comunidade ou pedidos de feedback
- Conversas sobre próximos releases
- Dúvidas gerais sobre o projeto
- Compartilhar sua experiência com o Microsandbox

### Contribuindo com código

Você não precisa ser expert em programação para contribuir com código! Segue um passo a passo:

1. **Ache algo em que trabalhar**
   - Navegue pelas issues existentes ou abra uma nova se tiver uma ideia

2. **Entre na conversa**
   - Comente na issue em que quer trabalhar
   - Isso evita trabalho duplicado e permite receber feedback da comunidade
   - Sem timidez — somos gente boa!

3. **Faça um fork do repositório**
   - Cria sua própria cópia do código para trabalhar
   - Novo em fork? Saiba mais [aqui][forking]

4. **Configure assinatura de commits**
   - Isso comprova que os commits são seus
   - Siga [este guia][commit-signing]
   - Já fez commits sem assinar? [Veja como corrigir][post-signing]

5. **Clone o repositório**
   - Traz seu fork para a máquina local
   - Precisa de ajuda? Veja [este guia][git-clone]

6. **Configure e compile o projeto**
   - Siga as instruções no **[DEVELOPMENT.md](./DEVELOPMENT.pt-BR.md)** para instalar pré-requisitos, compilar o projeto e configurar o ambiente
   - Se algo der errado, lembre-se: estamos a um e-mail de distância!

7. **Faça suas alterações**
   - Abra seu editor favorito e programe
   - Recompile e instale com `just build && just install`
   - Teste com `cargo test --workspace`
   - Faça pausas e divirta-se!

8. **Escreva testes**
   - Para funcionalidades novas: testes que protejam sua feature de quebrar
   - Para correções de bug: testes que garantam que o bug não volta
   - Considere testes de documentação mostrando como usar a feature
   - Não precisa acertar tudo de primeira no teste — podemos orientar

9. **Siga as diretrizes de código**
   - Veja nosso [guia de estilo Rust][rust-style-guide]
   - Código claro e legível ajuda quem vier depois
   - Na dúvida, faça o melhor que puder e mencione na PR

10. **Faça commit e push**
    - Envie seu trabalho para o seu repositório no GitHub
    - Usamos [conventional commits][conventional-commits] para as mensagens
    - Novo em Git? Veja [fazer commit][committing-code] e [push][push-remote]
    - Os hooks de pre-commit rodam automaticamente

11. **Abra um pull request**
    - É assim que seu código entra no projeto principal
    - Vamos revisar e podemos sugerir mudanças
    - Toda PR nos ajuda a crescer — desde já, obrigado!
    - Saiba mais [sobre pull requests][prs]

## Obrigado!

Toda contribuição, por menor que seja, faz diferença. Agradecemos seu interesse em deixar o Microsandbox melhor para todo mundo!

[conventional-commits]: https://www.conventionalcommits.org/en/v1.0.0/
[committing-code]: https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/making-changes-in-a-branch/committing-and-reviewing-changes-to-your-project
[commit-signing]: https://www.freecodecamp.org/news/what-is-commit-signing-in-git/
[forking]: https://docs.github.com/en/get-started/quickstart/fork-a-repo
[gh-discussions]: https://docs.github.com/en/discussions
[git-clone]: https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository
[good-first-issues]: https://github.com/superradcompany/microsandbox/labels/good%20first%20issue
[post-signing]: https://dev.to/jmarhee/signing-existing-commits-with-gpg-5b58
[prs]: https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests
[push-remote]: https://docs.github.com/en/get-started/using-git/pushing-commits-to-a-remote-repository
[rust-style-guide]: https://rust-lang.github.io/api-guidelines/about.html
[test-org]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
[support-email]: mailto:development@superrad.company
