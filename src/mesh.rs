use std::fs;
use std::io::BufReader;
use obj::{load_obj, Obj};

pub struct Mesh {
    index_list: std::vec::Vec<u16>,
    vertex_list: std::vec::Vec<obj::Vertex>
}

impl Mesh {

    pub fn new() -> Mesh {
        let il = Vec::new();
        let vl = Vec::new();
        Mesh {
            index_list: il,
            vertex_list: vl
        }
    }

    pub fn insta_load(&mut self, adress: String) -> Mesh {
        
        let il = Vec::new();
        let vl = Vec::new();
        let mut result = Mesh {
            index_list: il,
            vertex_list: vl
        };

        // ejemplo  "resources/meshes/abstract.obj"
        //let err_file: &str = ("### No se encontro el archivo.")
        let mut err_file: String = String::from("### No se encontro el archivo ");
        err_file.push_str(&adress);

        let mut err_load: String = String::from("### No se pudo cargar el objeto ");
        err_load.push_str(&adress);

        let input = BufReader::new(fs::File::open(adress).expect(&err_file));
        let obj: Obj = load_obj(input).expect(&err_load);
        let lista_indices = obj.indices;
        let lista_vertices = obj.vertices;
        
        std::mem::replace(&mut result.index_list,lista_indices);
        std::mem::replace(&mut result.vertex_list,lista_vertices);

        return result;
    }

    pub fn load(&mut self, adress: String){
        // ejemplo  "resources/meshes/abstract.obj"
        //let err_file: &str = ("### No se encontro el archivo.")
        let mut err_file: String = String::from("### No se encontro el archivo ");
        err_file.push_str(&adress);

        let mut err_load: String = String::from("### No se pudo cargar el objeto ");
        err_load.push_str(&adress);

        let input = BufReader::new(fs::File::open(adress).expect(&err_file));
        let obj: Obj = load_obj(input).expect(&err_load);
        let lista_indices = obj.indices;
        let lista_vertices = obj.vertices;
        
        std::mem::replace(&mut self.index_list,lista_indices);
        std::mem::replace(&mut self.vertex_list,lista_vertices);
    }
}