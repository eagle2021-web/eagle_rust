1.antd引入报错可能是版本问题
2.ant不能引入scss，需要引入css
```tsx
interface Props {
    num: number
}
interface State {
    
}
class A extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
  }
}
```