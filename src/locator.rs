use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        if self.is_empty() {
            self.push(ele);
          }
          else{
            self.push(ele);
            let ele_idx = self.len() - 1;
            // swap element with parent if it is smaller
            while self[ele_idx] < self[(ele_idx - 1)/2]{
              self.swap(ele_idx, (ele_idx - 1)/2);
            }
          }
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {return None}
      let mut length = self.len();
      self.swap(0, length - 1);
      let removed = self.pop();
      length -= 1;
      let mut curr_idx = 0;
    //   as long as there are children this block will swap the last element with the largest child
      while curr_idx * 2 + 1 < length  {
        if curr_idx * 2 + 2 >= length {
          if self[curr_idx] > self[curr_idx * 2 + 1]{
            self.swap(curr_idx, curr_idx * 2 + 1);
          }
          break
        }

        if self[curr_idx] > self[curr_idx * 2 +1] || self[curr_idx] > self[curr_idx *2+1]{
          if self[curr_idx * 2 + 1] <= self[curr_idx * 2 + 2]{
            self.swap(curr_idx, curr_idx * 2 + 1);
            curr_idx = curr_idx *2+1
          }
          else{
            self.swap(curr_idx, curr_idx * 2 + 2);
            curr_idx = curr_idx *2+2
          }
        }
        else{break}
      }
      return removed
    }
    

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {return None}
        Some(&self[0])
    }

}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    let mut stark_distances:Vec<i32> = Vec::new();
    let mut curr_enemy_distances:Vec<i32> = Vec::new();
    let mut closest_enemy = "";
    let mut closest_ally = "";
    let mut allies_list = Vec::new();
    allies_list = allies.keys().cloned().collect();
    
    println!("{:?}", allies_list);
    for enemy in enemies.keys() {
        stark_distances.enqueue(distance(*(allies.get(&"Stark".to_string()).unwrap()), *(enemies.get(*enemy).unwrap())));
        if *stark_distances.peek().unwrap() == distance(*(allies.get(&"Stark".to_string()).unwrap()), *(enemies.get(*enemy).unwrap())){
            closest_enemy = *enemy;
        }
    }
    while !stark_distances.is_empty(){
        for ally in &allies_list{
            curr_enemy_distances.enqueue(distance(*(allies.get(*ally).unwrap()), *(enemies.get(&closest_enemy.to_string()).unwrap())));
            if *curr_enemy_distances.peek().unwrap() == distance(*(allies.get(*ally).unwrap()), *(enemies.get(&closest_enemy.to_string()).unwrap())){
                closest_ally = *ally;
            }
        }

        if curr_enemy_distances.peek().unwrap() == stark_distances.peek().unwrap(){
            return (closest_enemy, enemies.get(&closest_enemy.to_string()).unwrap().0, enemies.get(&closest_enemy.to_string()).unwrap().1)
        }

        let index = allies_list.iter().position(|x| *x == closest_ally).unwrap();
        allies_list.remove(index);

        
        stark_distances.dequeue();
        for enemy in enemies.keys() {
            if *stark_distances.peek().unwrap() == distance(*(allies.get(&"Stark".to_string()).unwrap()), *(enemies.get(*enemy).unwrap())){
                closest_enemy = *enemy;
            }
        }
        curr_enemy_distances.clear()
        
    }
    (&"", 0 ,0)
}



