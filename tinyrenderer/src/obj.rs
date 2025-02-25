use std::fs;
use glam::*;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, space1, digit1},
    number::complete::float,
    combinator::map_res,
    sequence::tuple,
    multi::separated_list0,
    IResult,
};

// each .obj face has 3 sets of 3 vertices; actual vertices, textures, and normals
#[derive(Clone)]
pub struct ObjFace {
    pub vertices: [Vec3; 3],
    pub texture_vertices: [Vec3; 3],
    pub normals: [Vec3; 3]
}

// parse the object from file
// returns tuple of 3 Vec3; the face vertices, texture vertices, and normal vectors
pub fn parse_obj(filepath: &str) -> Vec<ObjFace> { 
    let contents = fs::read_to_string(filepath)
        .expect(&format!("No such file at this filepath: {filepath}")[..]);

    let mut vertex_coords = Vec::new();
    let mut normal_coords = Vec::new();
    let mut texture_coords = Vec::new();
    let mut obj_faces = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            match parse_vertex(&line) {
                Ok((_, coord)) =>vertex_coords.push(coord),
                Err(_) => continue
            }
        }

        else if line.starts_with("vt ") {
            match parse_texture(&line) {
                Ok((_, coord)) => texture_coords.push(coord),
                Err(_) => continue
            }
        }

        else if line.starts_with("vn ") {
            match parse_normal(&line) {
                Ok((_, coord)) => normal_coords.push(coord),
                Err(_) => continue
            }
        }

        else if line.starts_with("f ") {
            match parse_face(&line, &vertex_coords, &texture_coords, &normal_coords) {
                Ok((_, obj_face)) => obj_faces.push(obj_face),
                Err(_) => continue
            }
        }
    }
    obj_faces   
}

// face parsing
// each face has 3 components divided by "/", corresponding to face, texture, and normal indices.
// NOTE: indices in .obj files start from 1, hence why we must subtract 1 before using them.
fn parse_face<'a>(
    input: &'a str, 
    vertex_coords: &Vec<Vec3>, 
    texture_coords: &Vec<Vec3>,
    normal_coords: &Vec<Vec3>
) -> IResult<&'a str, ObjFace> {
    let (input, _) = char('f')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, v1_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v2_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v3_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    
    let vertices = [vertex_coords[v1_vec[0]-1], vertex_coords[v2_vec[0]-1], vertex_coords[v3_vec[0]-1]];
    let texture_vertices = [texture_coords[v1_vec[1]-1], texture_coords[v2_vec[1]-1], texture_coords[v3_vec[1]-1]];
    let normals = [normal_coords[v1_vec[2]-1], normal_coords[v2_vec[2]-1], normal_coords[v3_vec[2]-1]];

    Ok((input, ObjFace { vertices, texture_vertices, normals }))
}

// vertex parsing
fn parse_vertex(input: &str) -> IResult<&str, Vec3> {
    let (input, _) = char('v')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;
    Ok((input, Vec3 { x, y, z }))
}

// texture parsing
fn parse_texture(input: &str) -> IResult<&str, Vec3> {
    let (input, _) = tag("vt")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;
    
    Ok((input, Vec3 { x, y, z })) // Note: z seems to always be 0, but we'll store it anyway just in case
}   

// normal vertex parsing
fn parse_normal(input: &str) -> IResult<&str, Vec3> {
    let (input, _) = tag("vn")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;

    Ok((input, Vec3 { x, y, z }))
}