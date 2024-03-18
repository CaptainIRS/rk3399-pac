#[doc = "Register `PI_REG_75` reader"]
pub type R = crate::R<PiReg75Spec>;
#[doc = "Register `PI_REG_75` writer"]
pub type W = crate::W<PiReg75Spec>;
#[doc = "Field `PI_RDLVL_SEQ_EN` reader - Specifies the pattern, format, and MPR for data eye training."]
pub type PiRdlvlSeqEnR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_SEQ_EN` writer - Specifies the pattern, format, and MPR for data eye training."]
pub type PiRdlvlSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_RDLVL_GATE_SEQ_EN` reader - Specifies the pattern, format and MPR for gate training."]
pub type PiRdlvlGateSeqEnR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_SEQ_EN` writer - Specifies the pattern, format and MPR for gate training."]
pub type PiRdlvlGateSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_RDLVL_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
pub type PiRdlvlPeriodicR = crate::BitReader;
#[doc = "Field `PI_RDLVL_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
pub type PiRdlvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_ON_SREF_EXIT` reader - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_RDLVL_ON_SREF_EXIT` writer - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Specifies the pattern, format, and MPR for data eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_seq_en(&self) -> PiRdlvlSeqEnR {
        PiRdlvlSeqEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Specifies the pattern, format and MPR for gate training."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_seq_en(&self) -> PiRdlvlGateSeqEnR {
        PiRdlvlGateSeqEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_periodic(&self) -> PiRdlvlPeriodicR {
        PiRdlvlPeriodicR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_on_sref_exit(&self) -> PiRdlvlOnSrefExitR {
        PiRdlvlOnSrefExitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Specifies the pattern, format, and MPR for data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_seq_en(&mut self) -> PiRdlvlSeqEnW<PiReg75Spec> {
        PiRdlvlSeqEnW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Specifies the pattern, format and MPR for gate training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_seq_en(&mut self) -> PiRdlvlGateSeqEnW<PiReg75Spec> {
        PiRdlvlGateSeqEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_periodic(&mut self) -> PiRdlvlPeriodicW<PiReg75Spec> {
        PiRdlvlPeriodicW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_on_sref_exit(&mut self) -> PiRdlvlOnSrefExitW<PiReg75Spec> {
        PiRdlvlOnSrefExitW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_75::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_75::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg75Spec;
impl crate::RegisterSpec for PiReg75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_75::R`](R) reader structure"]
impl crate::Readable for PiReg75Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_75::W`](W) writer structure"]
impl crate::Writable for PiReg75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_75 to value 0"]
impl crate::Resettable for PiReg75Spec {
    const RESET_VALUE: u32 = 0;
}
