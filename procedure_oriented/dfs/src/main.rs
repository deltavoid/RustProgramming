



fn display(n: i32, k: i32, a: &[usize], vis: &[bool])
// fn display(n: i32, k: i32, a: &[i32; 10], vis: &[bool; 10])
{
    println!("n: {}", n);
    println!("k: {}", k);
    
    for x in a.iter()
    {
        println!("{} ", x);
    }
        
    for x in vis.iter()
    {
        println!("{}", x);
    }
} 

fn dfs(n: usize, k: usize, mut a: &mut [usize], mut vis: &mut [bool])
{
    if  k == n
    {   // for i in 0..(k as usize)
        for i in 0..k
        {   print!("{} ", a[i]);
        }
        println!();
    }
    else
    {
        for i in 0..n
        {   if  !vis[i]
            {   
                a[k] = i;
                vis[i] = true;
                dfs(n, k + 1, &mut a, &mut vis);
                vis[i] = false;
                a[k] = 0;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut a: [usize; 10] = [0; 10];
    let mut vis: [bool; 10] = [false; 10];

    display(3, 0, &a, &vis);

    dfs(4, 0, &mut a, &mut vis);

    
}
