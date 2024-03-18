#[doc = "Register `PI_REG_157` reader"]
pub type R = crate::R<PiReg157Spec>;
#[doc = "Register `PI_REG_157` writer"]
pub type W = crate::W<PiReg157Spec>;
#[doc = "Field `PI_TFC_F2` reader - Indicates the delay in PHY clock cycles from setting MR13.OP7 to any valid command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTfcF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TFC_F2` writer - Indicates the delay in PHY clock cycles from setting MR13.OP7 to any valid command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTfcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TCCD` reader - Indicates DRAM CAS-to-CAS value in cycles."]
pub type PiTccdR = crate::FieldReader;
#[doc = "Field `PI_TCCD` writer - Indicates DRAM CAS-to-CAS value in cycles."]
pub type PiTccdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TRTP_F0` reader - Indicates DRAM TRTP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrtpF0R = crate::FieldReader;
#[doc = "Field `PI_TRTP_F0` writer - Indicates DRAM TRTP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrtpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - Indicates the delay in PHY clock cycles from setting MR13.OP7 to any valid command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tfc_f2(&self) -> PiTfcF2R {
        PiTfcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - Indicates DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    pub fn pi_tccd(&self) -> PiTccdR {
        PiTccdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRTP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trtp_f0(&self) -> PiTrtpF0R {
        PiTrtpF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates the delay in PHY clock cycles from setting MR13.OP7 to any valid command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tfc_f2(&mut self) -> PiTfcF2W<PiReg157Spec> {
        PiTfcF2W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Indicates DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccd(&mut self) -> PiTccdW<PiReg157Spec> {
        PiTccdW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRTP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trtp_f0(&mut self) -> PiTrtpF0W<PiReg157Spec> {
        PiTrtpF0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 157\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_157::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_157::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg157Spec;
impl crate::RegisterSpec for PiReg157Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_157::R`](R) reader structure"]
impl crate::Readable for PiReg157Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_157::W`](W) writer structure"]
impl crate::Writable for PiReg157Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_157 to value 0"]
impl crate::Resettable for PiReg157Spec {
    const RESET_VALUE: u32 = 0;
}
