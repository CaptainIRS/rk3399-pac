#[doc = "Register `PI_REG_159` reader"]
pub type R = crate::R<PiReg159Spec>;
#[doc = "Register `PI_REG_159` writer"]
pub type W = crate::W<PiReg159Spec>;
#[doc = "Field `PI_TRAS_MAX_F0` reader - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f0\" of parameter name will be omitted when non DFS mode."]
pub type PiTrasMaxF0R = crate::FieldReader<u32>;
#[doc = "Field `PI_TRAS_MAX_F0` writer - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f0\" of parameter name will be omitted when non DFS mode."]
pub type PiTrasMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PI_TRAS_MIN_F0` reader - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrasMinF0R = crate::FieldReader;
#[doc = "Field `PI_TRAS_MIN_F0` writer - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrasMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:16 - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f0\" of parameter name will be omitted when non DFS mode."]
    #[inline(always)]
    pub fn pi_tras_max_f0(&self) -> PiTrasMaxF0R {
        PiTrasMaxF0R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tras_min_f0(&self) -> PiTrasMinF0R {
        PiTrasMinF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f0\" of parameter name will be omitted when non DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_max_f0(&mut self) -> PiTrasMaxF0W<PiReg159Spec> {
        PiTrasMaxF0W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_min_f0(&mut self) -> PiTrasMinF0W<PiReg159Spec> {
        PiTrasMinF0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_159::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_159::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg159Spec;
impl crate::RegisterSpec for PiReg159Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_159::R`](R) reader structure"]
impl crate::Readable for PiReg159Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_159::W`](W) writer structure"]
impl crate::Writable for PiReg159Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_159 to value 0"]
impl crate::Resettable for PiReg159Spec {
    const RESET_VALUE: u32 = 0;
}
