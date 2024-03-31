#[doc = "Register `GDBGBMU` reader"]
pub type R = crate::R<GdbgbmuSpec>;
#[doc = "Register `GDBGBMU` writer"]
pub type W = crate::W<GdbgbmuSpec>;
#[doc = "Field `BMU_CCU` reader - BMU_CCU Debug information\n\nBMU_CCU Debug information"]
pub type BmuCcuR = crate::FieldReader;
#[doc = "Field `BMU_DCU` reader - BMU_DCU Debug information\n\nBMU_DCU Debug information"]
pub type BmuDcuR = crate::FieldReader;
#[doc = "Field `BMU_BCU` reader - BMU_BCU Debug information\n\nBMU_BCU Debug information"]
pub type BmuBcuR = crate::FieldReader<u32>;
#[doc = "Field `BMU_BCU` writer - BMU_BCU Debug information\n\nBMU_BCU Debug information"]
pub type BmuBcuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - BMU_CCU Debug information\n\nBMU_CCU Debug information"]
    #[inline(always)]
    pub fn bmu_ccu(&self) -> BmuCcuR {
        BmuCcuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - BMU_DCU Debug information\n\nBMU_DCU Debug information"]
    #[inline(always)]
    pub fn bmu_dcu(&self) -> BmuDcuR {
        BmuDcuR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - BMU_BCU Debug information\n\nBMU_BCU Debug information"]
    #[inline(always)]
    pub fn bmu_bcu(&self) -> BmuBcuR {
        BmuBcuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - BMU_BCU Debug information\n\nBMU_BCU Debug information"]
    #[inline(always)]
    #[must_use]
    pub fn bmu_bcu(&mut self) -> BmuBcuW<GdbgbmuSpec> {
        BmuBcuW::new(self, 8)
    }
}
#[doc = "Global Debug BMU Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbgbmu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdbgbmu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdbgbmuSpec;
impl crate::RegisterSpec for GdbgbmuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdbgbmu::R`](R) reader structure"]
impl crate::Readable for GdbgbmuSpec {}
#[doc = "`write(|w| ..)` method takes [`gdbgbmu::W`](W) writer structure"]
impl crate::Writable for GdbgbmuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDBGBMU to value 0"]
impl crate::Resettable for GdbgbmuSpec {
    const RESET_VALUE: u32 = 0;
}
