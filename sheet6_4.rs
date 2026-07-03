#[derive(Copy, Clone, Debug, PartialEq)]
struct PublicStreetlight<'a>{
    id: &'a str,
    on: bool,
    burn_out: bool,
}

impl<'a> PublicStreetlight<'a>{
    fn new(id:&'a str,on:bool,burn_out:bool)-> Self{
        PublicStreetlight { id,on,burn_out }
    }

    fn default()->Self{
        Self::new("",false,false)
    }
}

struct PublicIlluminaiton<'a>{
    lights: Vec<PublicStreetlight<'a>>,
}

impl<'a> PublicIlluminaiton<'a>{

    fn new(p0:Vec<PublicStreetlight<'a>>) -> Self{
        Self{lights:p0}
    }

    fn default()->Self{
        Self::new(Vec::new())
    }
}

impl<'a> Iterator for PublicIlluminaiton<'a>{
    type Item = PublicStreetlight<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.lights.iter().enumerate().find(|&x| x.1.burn_out == true );
        match a{
            Some((idx,_)) => {
                let b = self.lights[idx];
                self.lights.remove(idx);
                Some(b)
            }
            None => None,
        }
    }
}
