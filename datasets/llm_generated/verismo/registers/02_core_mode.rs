// Variation: CPU Core Mode Tracking
// From: source/verismo/src/registers/core_perm_s.rs (CoreMode)
// Demonstrates: Tracking CPU state including VMPL, run status, and exit count

use vstd::prelude::*;

verus! {

pub struct CoreMode {
    pub cpu: usize,
    pub run: bool,
    pub vmpl: usize,
    pub count: usize,
}

impl CoreMode {
    pub fn new(cpu: usize, vmpl: usize) -> (result: Self)
        ensures
            result.cpu == cpu,
            result.vmpl == vmpl,
            result.run == false,
            result.count == 0,
    {
        CoreMode {
            cpu,
            run: false,
            vmpl,
            count: 0,
        }
    }

    pub fn start_running(&mut self)
        ensures
            self.run,
            self.cpu == old(self).cpu,
            self.vmpl == old(self).vmpl,
            self.count == old(self).count,
    {
        self.run = true;
    }

    pub fn stop_running(&mut self)
        ensures
            !self.run,
            self.cpu == old(self).cpu,
            self.vmpl == old(self).vmpl,
            self.count == old(self).count,
    {
        self.run = false;
    }

    pub fn increment_exit_count(&mut self)
        requires
            old(self).count < usize::MAX,
        ensures
            self.count == old(self).count + 1,
            self.cpu == old(self).cpu,
            self.vmpl == old(self).vmpl,
            self.run == old(self).run,
    {
        self.count = self.count + 1;
    }

    pub fn is_running(&self) -> (result: bool)
        ensures
            result == self.run,
    {
        self.run
    }

    pub fn get_cpu(&self) -> (result: usize)
        ensures
            result == self.cpu,
    {
        self.cpu
    }

    pub fn get_vmpl(&self) -> (result: usize)
        ensures
            result == self.vmpl,
    {
        self.vmpl
    }

    pub fn get_exit_count(&self) -> (result: usize)
        ensures
            result == self.count,
    {
        self.count
    }

    pub open spec fn is_at_vmpl(&self, vmpl_level: usize) -> bool {
        self.vmpl == vmpl_level
    }

    pub fn check_at_vmpl(&self, vmpl_level: usize) -> (result: bool)
        ensures
            result <==> self.is_at_vmpl(vmpl_level),
    {
        self.vmpl == vmpl_level
    }

    pub open spec fn matches_cpu(&self, cpu_id: usize) -> bool {
        self.cpu == cpu_id
    }

    pub fn check_matches_cpu(&self, cpu_id: usize) -> (result: bool)
        ensures
            result <==> self.matches_cpu(cpu_id),
    {
        self.cpu == cpu_id
    }
}

pub struct CoreCollection {
    pub cores: Ghost<Map<usize, CoreMode>>,
    pub bsp: usize,
}

impl CoreCollection {
    pub fn new(bsp: usize) -> (result: Self)
        ensures
            result.bsp == bsp,
            result.cores@.len() == 0,
    {
        CoreCollection {
            cores: Ghost(Map::empty()),
            bsp,
        }
    }

    pub open spec fn contains_core(&self, cpu: usize, vmpl: usize) -> bool {
        self.cores@.contains_key(cpu) && self.cores@[cpu].vmpl == vmpl && self.cores@[cpu].cpu == cpu
    }

    pub open spec fn all_cores_at_vmpl(&self, vmpl: usize) -> bool {
        forall|i: usize| self.cores@.contains_key(i) ==> #[trigger] self.cores@[i].vmpl == vmpl
    }

    pub open spec fn all_ap_cores_wf(&self) -> bool {
        forall|i: usize| (i != self.bsp && self.cores@.contains_key(i)) ==> #[trigger]
            self.contains_core(i, 0)
    }

    pub open spec fn cores_in_range_wf(&self, min_cpu: usize) -> bool {
        forall|i: usize| (i != self.bsp && i >= min_cpu && self.cores@.contains_key(i)) ==> #[trigger]
            self.contains_core(i, 0)
    }
}

fn test_core_mode() {
    let mut core = CoreMode::new(1, 0);
    let cpu = core.get_cpu();
    let vmpl = core.get_vmpl();

    core.start_running();
    let is_running = core.is_running();

    core.increment_exit_count();
    let count = core.get_exit_count();

    core.stop_running();

    let at_vmpl0 = core.check_at_vmpl(0);
    let matches_cpu1 = core.check_matches_cpu(1);

    let collection = CoreCollection::new(0);

    proof {
        assert(collection.bsp == 0);
        assert(collection.cores@.len() == 0);
    }
}

} // verus!

fn main() {
    test_core_mode();
}
