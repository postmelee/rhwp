use rhwp::document_core::DocumentCore;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};
use serde_json::json;
use std::path::Path;
fn lines(node: &RenderNode, page: u32, out: &mut Vec<serde_json::Value>) {
    if let RenderNodeType::TextLine(info) = &node.node_type {out.push(json!({"page":page,"paragraph":info.para_index,"bbox":node.bbox}));}
    for child in &node.children {lines(child,page,out);}
}
fn save(core: &DocumentCore, dir: &Path, name: &str) {
    std::fs::write(dir.join(format!("{name}.hwp")),core.export_hwp_native().unwrap()).unwrap();
    let mut coords=Vec::new();
    for page in 0..core.page_count() {
        std::fs::write(dir.join(format!("{name}-p{}.svg",page+1)),core.render_page_svg_native(page).unwrap()).unwrap();
        lines(&core.build_page_render_tree(page).unwrap().root,page,&mut coords);
    }
    let paragraphs: Vec<_> = core.document().sections[0].paragraphs.iter().map(|p|json!({"text":p.text,"line_segs":p.line_segs})).collect();
    let reopened=DocumentCore::from_bytes(&core.export_hwp_native().unwrap()).unwrap();
    std::fs::write(dir.join(format!("{name}.json")),serde_json::to_vec_pretty(&json!({"pages":core.page_count(),"reopened_pages":reopened.page_count(),"paragraphs":paragraphs,"lines":coords})).unwrap()).unwrap();
    println!("{name}: live_pages={} reopened_pages={}",core.page_count(),reopened.page_count());
}
fn main() {
    let out=std::env::args().nth(1).expect("output directory"); let dir=Path::new(&out); std::fs::create_dir_all(dir).unwrap();
    for by_id in [false,true] {
        let mut core=DocumentCore::from_bytes(&std::fs::read("saved/blank2010.hwp").unwrap()).unwrap();
        core.insert_text_native(0,0,0,"Title").unwrap();
        core.apply_char_format_native(0,0,0,5,r#"{"fontSize":2000}"#).unwrap();
        for (i,text) in ["A","B","","C","D"].iter().enumerate() {
            core.insert_paragraph_native(0,i+1).unwrap();
            if !text.is_empty() {core.insert_text_native(0,i+1,0,text).unwrap();}
        }
        if !by_id {save(&core,dir,"before-format");}
        core.apply_char_format_native(0,2,0,1,r#"{"fontSize":1300,"bold":true}"#).unwrap();
        if by_id {let id=core.document().sections[0].paragraphs[2].char_shapes[0].char_shape_id; core.set_char_shape_id_native(0,4,0,1,id).unwrap();}
        else {core.apply_char_format_native(0,4,0,1,r#"{"fontSize":1300,"bold":true}"#).unwrap();}
        let name=if by_id {"set-shape-id"}else{"apply-char-format"}; save(&core,dir,name);
        core.delete_paragraph_native(0,3).unwrap(); save(&core,dir,&format!("{name}-delete-empty"));
    }
    let bytes=std::fs::read("samples/hwp3-sample16-hwp5.hwp").unwrap();
    let original=DocumentCore::from_bytes(&bytes).unwrap(); let paras=&original.document().sections[0].paragraphs;
    let boundary=(1..paras.len()).find(|&i| {
        matches!((paras[i-1].line_segs.last(),paras[i].line_segs.first()),(Some(a),Some(b)) if a.vertical_pos>5000 && b.vertical_pos==0 && a.tag&0x80000000==0 && b.tag&0x80000000==0 && !paras[i-1].text.is_empty() && !paras[i].text.is_empty())
    });
    println!("stored_boundary={boundary:?}");
    if let Some(i)=boundary {
        let mut observations=Vec::new();
        for pi in [i-1,i] {for by_id in [false,true] {
            let mut core=DocumentCore::from_bytes(&bytes).unwrap(); let para=&core.document().sections[0].paragraphs[pi];
            let id=para.char_shapes.first().map(|r|r.char_shape_id).unwrap_or(0); let size=core.document().doc_info.char_shapes[id as usize].base_size;
            if by_id {core.set_char_shape_id_native(0,pi,0,1,id).unwrap();}
            else {core.apply_char_format_native(0,pi,0,1,&format!("{{\"fontSize\":{size}}}")).unwrap();}
            let mut coords=Vec::new(); for page in 0..core.page_count() {lines(&core.build_page_render_tree(page).unwrap().root,page,&mut coords);}
            observations.push(json!({"edited_para":pi,"by_id":by_id,"pages":core.page_count(),"boundary_vpos":core.document().sections[0].paragraphs[i].line_segs.first().map(|s|s.vertical_pos),"nearby_lines":coords.into_iter().filter(|v|v["paragraph"].as_u64().is_some_and(|p|p>=i as u64-1 && p<=i as u64+1)).collect::<Vec<_>>()}));
        }}
        std::fs::write(dir.join("stored-boundary.json"),serde_json::to_vec_pretty(&observations).unwrap()).unwrap();
    }
}
