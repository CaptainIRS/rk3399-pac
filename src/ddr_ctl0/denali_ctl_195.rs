#[doc = "Register `DENALI_CTL_195` reader"]
pub type R = crate::R<DenaliCtl195Spec>;
#[doc = "Register `DENALI_CTL_195` writer"]
pub type W = crate::W<DenaliCtl195Spec>;
#[doc = "Field `NUM_Q_ENTRIES_ACT_DISABLE` reader - Number of queue entries in which ACT requests will be disabled. Setting to X will disable ACT requests from the X entries lowest in the command queue."]
pub type NumQEntriesActDisableR = crate::FieldReader;
#[doc = "Field `NUM_Q_ENTRIES_ACT_DISABLE` writer - Number of queue entries in which ACT requests will be disabled. Setting to X will disable ACT requests from the X entries lowest in the command queue."]
pub type NumQEntriesActDisableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWAP_EN` reader - Enable command swapping logic in execution unit. Set to 1 to enable."]
pub type SwapEnR = crate::BitReader;
#[doc = "Field `SWAP_EN` writer - Enable command swapping logic in execution unit. Set to 1 to enable."]
pub type SwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_RD_INTERLEAVE` reader - Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
pub type DisableRdInterleaveR = crate::BitReader;
#[doc = "Field `DISABLE_RD_INTERLEAVE` writer - Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
pub type DisableRdInterleaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INHIBIT_DRAM_CMD` reader - Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
pub type InhibitDramCmdR = crate::FieldReader;
#[doc = "Field `INHIBIT_DRAM_CMD` writer - Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
pub type InhibitDramCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Number of queue entries in which ACT requests will be disabled. Setting to X will disable ACT requests from the X entries lowest in the command queue."]
    #[inline(always)]
    pub fn num_q_entries_act_disable(&self) -> NumQEntriesActDisableR {
        NumQEntriesActDisableR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Enable command swapping logic in execution unit. Set to 1 to enable."]
    #[inline(always)]
    pub fn swap_en(&self) -> SwapEnR {
        SwapEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
    #[inline(always)]
    pub fn disable_rd_interleave(&self) -> DisableRdInterleaveR {
        DisableRdInterleaveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
    #[inline(always)]
    pub fn inhibit_dram_cmd(&self) -> InhibitDramCmdR {
        InhibitDramCmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of queue entries in which ACT requests will be disabled. Setting to X will disable ACT requests from the X entries lowest in the command queue."]
    #[inline(always)]
    #[must_use]
    pub fn num_q_entries_act_disable(&mut self) -> NumQEntriesActDisableW<DenaliCtl195Spec> {
        NumQEntriesActDisableW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable command swapping logic in execution unit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn swap_en(&mut self) -> SwapEnW<DenaliCtl195Spec> {
        SwapEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
    #[inline(always)]
    #[must_use]
    pub fn disable_rd_interleave(&mut self) -> DisableRdInterleaveW<DenaliCtl195Spec> {
        DisableRdInterleaveW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
    #[inline(always)]
    #[must_use]
    pub fn inhibit_dram_cmd(&mut self) -> InhibitDramCmdW<DenaliCtl195Spec> {
        InhibitDramCmdW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_195::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_195::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl195Spec;
impl crate::RegisterSpec for DenaliCtl195Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_195::R`](R) reader structure"]
impl crate::Readable for DenaliCtl195Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_195::W`](W) writer structure"]
impl crate::Writable for DenaliCtl195Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_195 to value 0"]
impl crate::Resettable for DenaliCtl195Spec {
    const RESET_VALUE: u32 = 0;
}
