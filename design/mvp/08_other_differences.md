# 08 Other Differences

## Removal of Legacy Syntax

- `with`: no one should be using this
- `eval`: no one should be using this
- `ternary`: superseded by `if`-`else` bing converted to an expression
- `switch`-`case`: superseded by pattern matching
- `function`: arrow syntax is used exclusively. This will make supporting
  generators interesting.

## No Default imports/exports

Default imports/exports make it difficult to maintain consistent naming of
variables across large projects. You'll still be able to import `default` using
a named import to support TypeScript interop.

## Autobinding

One of the confusing aspects of using JavaScript is passing methods as
callbacks. To avoid this confusion when a method is passed as a callback,
Crochet will autobind it.

```tsx
// autobinding.crochet
class MyComponents extends React.Component {
  constructor(props) {
    super(props);
    this.state = { count: 0 };
  }

  handleClick() {
    this.state.count++;
  }

  render() {
    return (
      <div>
        <h1>Click count: {this.state.count}</h1>
        <button onClick={this.handleClick}>Click me!</button>
      </div>
    );
  }
}

// autobinding.js
class MyComponents extends React.Component {
  constructor(props) {
    super(props);
    this.state = { count: 0 };
  }

  handleClick() {
    this.state.count++;
  }

  render() {
    return (
      <div>
        <h1>Click count: {this.state.count}</h1>
        <button onClick={(...args) => this.handleClick(...args)}>
          Click me!
        </button>
      </div>
    );
  }
}
```

NOTE: The reason we don't use `.bind` in the JavaScript output is that it's
slow.
