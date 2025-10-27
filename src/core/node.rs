use super::Timeline;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

type Participants =  HashSet<String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Project {
        id: Uuid,
        name: String,
        link: Option<String>,
        timeline: Option<Timeline>,
        owner: Option<String>,
        participants: Option<Participants>,
    },
    Spec {
        id: Uuid,
        name: String,
        link: Option<String>,
        owner: Option<String>,
    },
    Epic {
        id: Uuid,
        name: String,
        link: Option<String>,
        timeline: Timeline,
        points: Option<u32>,
        owner: Option<String>,
        participants: Option<Participants>,
    },
    UserStory {
        id: Uuid,
        name: String,
        link: Option<String>,
        timeline: Timeline,
        points: Option<u32>,
        owner: Option<String>,
    },
    Tasks {
        id: Uuid,
        name: String,
        link: Option<String>,
        timeline: Timeline,
        points: Option<u32>,
        owner: Option<String>,
    },
}

impl Node{
    pub fn get_id(&self) -> Uuid{
        match self{
            Node::Project { id,..}|
            Node::Spec{id,..}|
            Node::Epic{id,..} |
            Node::UserStory { id, ..}|
            Node::Tasks { id,..} => {
                *id
            }
        }
    }

    pub fn get_name(&self) -> &str{
        match self{
            Node::Project {name,..}|
            Node::Spec{name,..}|
            Node::Epic{name,..} |
            Node::UserStory {name, ..}|
            Node::Tasks {name,..} => {
                name
            }
        }
    }

    pub fn set_id(&mut self, uid: Uuid){
        match self{
                Node::Project{id,..} |
                Node::Spec{id,..}|
                Node::Epic{id,..} |
                Node::UserStory { id, ..}|
                Node::Tasks { id,..} => {
                    *id = uid;
                }
        }
    }

    pub fn set_name(&mut self, new_name: String){
        match self{
                Node::Project{name,..}|
                Node::Spec{name,..}|
                Node::Epic{name,..} |
                Node::UserStory {name,..}|
                Node::Tasks {name,..} => {
                    *name = new_name;
                }
        }
    }

    pub fn set_link(&mut self, new_link: String){
        match self{
                Node::Project{link,..} |
                Node::Spec{link,..}|
                Node::Epic{link,..} |
                Node::UserStory {link,..}|
                Node::Tasks {link,..} => {
                    *link = Some(new_link);
                }
        }
    }

    pub fn set_timeline(&mut self, new_timeline: Timeline){
        match self{
                Node::Project{timeline,..} =>{
                    *timeline = Some(new_timeline)
                }
                Node::Spec{..} => {}
                Node::Epic{timeline,..} |
                Node::UserStory {timeline,..}|
                Node::Tasks {timeline,..} => {
                    *timeline = new_timeline
                }
        }
    }

    pub fn set_owner(&mut self, new_owner: String){
        match self{
                Node::Project{owner,..} |
                Node::Spec{owner,..}|
                Node::Epic{owner,..} |
                Node::UserStory {owner,..}|
                Node::Tasks {owner,..} => {
                    *owner = Some(new_owner);
                }
        }
    }

    pub fn add_participant(&mut self, participant: String)->Result<(),&'static str>{
        match self{
                Node::Project{participants,..} |
                Node::Epic{participants,..} => {
                        participants.get_or_insert_with(HashSet::new).insert(participant);
                        Ok(())
                }
                _ => {
                    Err("This node type does not support participants")
                }
        }
    }

    pub fn remove_participant(&mut self, participant: &str)->Result<(), &'static str>{
        match self{
            Node::Project{participants,..} |
            Node::Epic{participants,..} => {
                match participants{
                    Some(hs) => {
                        if hs.remove(participant) {
                           Ok(())  
                        }else{
                            Err("participant does not exist")
                        }
                    }
                    _ => {
                        Err("Participants for this node are empty")
                    }
                }
            }
            _ => {
                Err("This node type does not support participants")
            }
        }
    }

    pub fn set_points(&mut self, new_points : u32)-> Result<(),&'static str>{
        match self{
            Node::Epic{points,..}|
            Node::UserStory{points,..}|
            Node::Tasks{points,..}=> {
                *points = Some(new_points);
                Ok(())
            }
            _=>{
                Err("This node type does not contain points")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct NodeBuilder{
    id: Option<Uuid>,
    name: Option<String>,
    link: Option<String>,
    timeline: Option<Timeline>,
    owner: Option<String>,
    points : Option<u32>,
    participants: Option<Participants>, 
}

impl NodeBuilder{
    pub fn new() -> NodeBuilder{
        NodeBuilder::default()
    }

    pub fn with_id(mut self,id: Uuid)->Self{
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self,name: String)->Self{
        self.name = Some(name);
        self
    }

    pub fn with_link(mut self,link: String)->Self{
        self.link = Some(link);
        self
    }

    pub fn with_timeline(mut self,tl: Timeline)->Self{
        self.timeline = Some(tl);
        self
    }

    pub fn with_owner(mut self,owner: String)->Self{
        self.owner = Some(owner);
        self
    }

    pub fn with_points(mut self,points: u32)->Self{
        self.points = Some(points);
        self
    }

    pub fn with_participants(mut self, participants : Participants)->Self{
        self.participants = Some(participants);
        self
    }

    pub fn build_project(self)->Result<Node, &'static str> {
        let id = self.id.ok_or("Failed to build project - missing project id")?;
        let name = self.name.ok_or("Failed to build project - missing project name")?;
        
        Ok(Node::Project { 
            id, 
            name, 
            link: self.link, 
            timeline: self.timeline, 
            owner: self.owner, 
            participants: self.participants}) 
    }

    pub fn build_spec(self)->Result<Node, &'static str> {
        let id = self.id.ok_or("Failed to build Spec - missing Spec id")?;
        let name = self.name.ok_or("Failed to build Spec - missing Spec name")?;
        
        Ok(Node::Spec { id, name, link: self.link, owner: self.owner})
    }

    pub fn build_epic(self)->Result<Node, &'static str> {
        let id =  self.id.ok_or("Failed to build Epic - missing Epic id")?;
        let name =  self.name.ok_or("Failed to build Epic - missing Epic name")?;
        let timeline =  self.timeline.ok_or("Failed to build Epic - missing Epic timeline")?;

        Ok(Node::Epic { id, name, link: self.link, timeline, points: self.points, owner: self.owner, participants: self.participants })
    }

    pub fn build_userstory(self)->Result<Node, &'static str> {
        let id =  self.id.ok_or("Failed to build Userstory - missing Userstory id")?;
        let name =  self.name.ok_or("Failed to build Userstory - missing Userstory name")?;
        let timeline =  self.timeline.ok_or("Failed to build Userstory - missing Userstory timeline")?;

        Ok(Node::UserStory { id, name, link:self.link, timeline: timeline, points: self.points, owner: self.owner })
    }

    pub fn build_tasks(self)->Result<Node, &'static str> {
        let id =  self.id.ok_or("Failed to build Tasks - missing Tasks id")?;
        let name =  self.name.ok_or("Failed to build Tasks - missing Tasks name")?;
        let timeline =  self.timeline.ok_or("Failed to build Tasks - missing Tasks timeline")?;

        Ok(Node::Tasks { id, name, link:self.link, timeline: timeline, points: self.points, owner: self.owner })
    }

}



