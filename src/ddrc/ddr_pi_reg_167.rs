#[doc = "Register `DDR_PI_REG_167` reader"]
pub type R = crate::R<DdrPiReg167Spec>;
#[doc = "Register `DDR_PI_REG_167` writer"]
pub type W = crate::W<DdrPiReg167Spec>;
#[doc = "Field `PI_TRAS_MAX_F2` reader - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f2\" of\n\nparameter name will be omitted when non DFS mode."]
pub type PiTrasMaxF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TRAS_MAX_F2` writer - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f2\" of\n\nparameter name will be omitted when non DFS mode."]
pub type PiTrasMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PI_TRAS_MIN_F2` reader - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTrasMinF2R = crate::FieldReader;
#[doc = "Field `PI_TRAS_MIN_F2` writer - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTrasMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:16 - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f2\" of\n\nparameter name will be omitted when non DFS mode."]
    #[inline(always)]
    pub fn pi_tras_max_f2(&self) -> PiTrasMaxF2R {
        PiTrasMaxF2R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tras_min_f2(&self) -> PiTrasMinF2R {
        PiTrasMinF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Indicates DRAM TRAS_MAX value in cycles. The suffix \"_f2\" of\n\nparameter name will be omitted when non DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_max_f2(&mut self) -> PiTrasMaxF2W<DdrPiReg167Spec> {
        PiTrasMaxF2W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRAS_MIN value in cycles. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_min_f2(&mut self) -> PiTrasMinF2W<DdrPiReg167Spec> {
        PiTrasMinF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 167\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_167::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_167::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg167Spec;
impl crate::RegisterSpec for DdrPiReg167Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_167::R`](R) reader structure"]
impl crate::Readable for DdrPiReg167Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_167::W`](W) writer structure"]
impl crate::Writable for DdrPiReg167Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_167 to value 0"]
impl crate::Resettable for DdrPiReg167Spec {
    const RESET_VALUE: u32 = 0;
}
