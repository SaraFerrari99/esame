use std::marker::PhantomData;
trait CompileTimeNode{
    type LeftType: CompileTimeNode;
    type RightType: CompileTimeNode;
    fn is_none() -> bool;
}
struct NullNode{}
impl CompileTimeNode for NullNode{
    type LeftType = NullNode;
    type RightType = NullNode;
    fn is_none() -> bool {
        true
    }
}
struct Node<L: CompileTimeNode,R: CompileTimeNode>{
    left: PhantomData<L>,
    right: PhantomData<R>
}
impl<L: CompileTimeNode,R: CompileTimeNode> CompileTimeNode for Node<L,R>{
    type LeftType = L;
    type RightType = R;
    fn is_none() -> bool{ false }
}

fn count_nodes<T: CompileTimeNode>() -> usize{
    let mut count = 0;
    if !T::is_none(){
        count = 1;
        count += count_nodes::<T::LeftType>();
        count += count_nodes::<T::RightType>();
    }
    count
}

/*
// Caso ricorsivo: 1 (questo nodo) + il conteggio del ramo sinistro + il ramo destro
    fn count_nodes() -> usize {
        1 + L::count_nodes() + R::count_nodes()
    }
*/