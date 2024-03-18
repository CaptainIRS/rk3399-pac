#[doc = "Register `PI_REG_24` reader"]
pub type R = crate::R<PiReg24Spec>;
#[doc = "Register `PI_REG_24` writer"]
pub type W = crate::W<PiReg24Spec>;
#[doc = "Field `PI_INIT_WORK_FREQ` reader - Indicates the initial work frequency after initialization and initial leveling sequence."]
pub type PiInitWorkFreqR = crate::FieldReader;
#[doc = "Field `PI_INIT_WORK_FREQ` writer - Indicates the initial work frequency after initialization and initial leveling sequence."]
pub type PiInitWorkFreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_INIT_DFS_CALVL_ONLY` reader - Enables the initial leveling sequence loop that only CA training executes DFS."]
pub type PiInitDfsCalvlOnlyR = crate::BitReader;
#[doc = "Field `PI_INIT_DFS_CALVL_ONLY` writer - Enables the initial leveling sequence loop that only CA training executes DFS."]
pub type PiInitDfsCalvlOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_POWER_ON_SEQ_BYPASS_ARRAY` reader - Indicates the bypassed steps of power on sequence."]
pub type PiPowerOnSeqBypassArrayR = crate::FieldReader;
#[doc = "Field `PI_POWER_ON_SEQ_BYPASS_ARRAY` writer - Indicates the bypassed steps of power on sequence."]
pub type PiPowerOnSeqBypassArrayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_POWER_ON_SEQ_END_ARRAY` reader - Indicates the step that is the last step of the power-on sequence."]
pub type PiPowerOnSeqEndArrayR = crate::FieldReader;
#[doc = "Field `PI_POWER_ON_SEQ_END_ARRAY` writer - Indicates the step that is the last step of the power-on sequence."]
pub type PiPowerOnSeqEndArrayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - Indicates the initial work frequency after initialization and initial leveling sequence."]
    #[inline(always)]
    pub fn pi_init_work_freq(&self) -> PiInitWorkFreqR {
        PiInitWorkFreqR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enables the initial leveling sequence loop that only CA training executes DFS."]
    #[inline(always)]
    pub fn pi_init_dfs_calvl_only(&self) -> PiInitDfsCalvlOnlyR {
        PiInitDfsCalvlOnlyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Indicates the bypassed steps of power on sequence."]
    #[inline(always)]
    pub fn pi_power_on_seq_bypass_array(&self) -> PiPowerOnSeqBypassArrayR {
        PiPowerOnSeqBypassArrayR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the step that is the last step of the power-on sequence."]
    #[inline(always)]
    pub fn pi_power_on_seq_end_array(&self) -> PiPowerOnSeqEndArrayR {
        PiPowerOnSeqEndArrayR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicates the initial work frequency after initialization and initial leveling sequence."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_work_freq(&mut self) -> PiInitWorkFreqW<PiReg24Spec> {
        PiInitWorkFreqW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables the initial leveling sequence loop that only CA training executes DFS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_dfs_calvl_only(&mut self) -> PiInitDfsCalvlOnlyW<PiReg24Spec> {
        PiInitDfsCalvlOnlyW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates the bypassed steps of power on sequence."]
    #[inline(always)]
    #[must_use]
    pub fn pi_power_on_seq_bypass_array(&mut self) -> PiPowerOnSeqBypassArrayW<PiReg24Spec> {
        PiPowerOnSeqBypassArrayW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates the step that is the last step of the power-on sequence."]
    #[inline(always)]
    #[must_use]
    pub fn pi_power_on_seq_end_array(&mut self) -> PiPowerOnSeqEndArrayW<PiReg24Spec> {
        PiPowerOnSeqEndArrayW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg24Spec;
impl crate::RegisterSpec for PiReg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_24::R`](R) reader structure"]
impl crate::Readable for PiReg24Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_24::W`](W) writer structure"]
impl crate::Writable for PiReg24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_24 to value 0x0100"]
impl crate::Resettable for PiReg24Spec {
    const RESET_VALUE: u32 = 0x0100;
}
