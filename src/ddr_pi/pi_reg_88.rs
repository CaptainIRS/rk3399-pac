#[doc = "Register `PI_REG_88` reader"]
pub type R = crate::R<PiReg88Spec>;
#[doc = "Register `PI_REG_88` writer"]
pub type W = crate::W<PiReg88Spec>;
#[doc = "Field `PI_LPDDR4_RDLVL_PATTERN_11` reader - Indicates user-defined LPDDR4 read data pattern 11."]
pub type PiLpddr4RdlvlPattern11R = crate::FieldReader<u32>;
#[doc = "Field `PI_LPDDR4_RDLVL_PATTERN_11` writer - Indicates user-defined LPDDR4 read data pattern 11."]
pub type PiLpddr4RdlvlPattern11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates user-defined LPDDR4 read data pattern 11."]
    #[inline(always)]
    pub fn pi_lpddr4_rdlvl_pattern_11(&self) -> PiLpddr4RdlvlPattern11R {
        PiLpddr4RdlvlPattern11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates user-defined LPDDR4 read data pattern 11."]
    #[inline(always)]
    #[must_use]
    pub fn pi_lpddr4_rdlvl_pattern_11(&mut self) -> PiLpddr4RdlvlPattern11W<PiReg88Spec> {
        PiLpddr4RdlvlPattern11W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_88::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_88::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg88Spec;
impl crate::RegisterSpec for PiReg88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_88::R`](R) reader structure"]
impl crate::Readable for PiReg88Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_88::W`](W) writer structure"]
impl crate::Writable for PiReg88Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_88 to value 0"]
impl crate::Resettable for PiReg88Spec {
    const RESET_VALUE: u32 = 0;
}
