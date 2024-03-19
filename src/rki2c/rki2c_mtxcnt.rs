#[doc = "Register `RKI2C_MTXCNT` reader"]
pub type R = crate::R<Rki2cMtxcntSpec>;
#[doc = "Register `RKI2C_MTXCNT` writer"]
pub type W = crate::W<Rki2cMtxcntSpec>;
#[doc = "Field `MTXCNT` reader - master transmit count\n\n6 bits counter"]
pub type MtxcntR = crate::FieldReader;
#[doc = "Field `MTXCNT` writer - master transmit count\n\n6 bits counter"]
pub type MtxcntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - master transmit count\n\n6 bits counter"]
    #[inline(always)]
    pub fn mtxcnt(&self) -> MtxcntR {
        MtxcntR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - master transmit count\n\n6 bits counter"]
    #[inline(always)]
    #[must_use]
    pub fn mtxcnt(&mut self) -> MtxcntW<Rki2cMtxcntSpec> {
        MtxcntW::new(self, 0)
    }
}
#[doc = "master transmit count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_mtxcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_mtxcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cMtxcntSpec;
impl crate::RegisterSpec for Rki2cMtxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_mtxcnt::R`](R) reader structure"]
impl crate::Readable for Rki2cMtxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_mtxcnt::W`](W) writer structure"]
impl crate::Writable for Rki2cMtxcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_MTXCNT to value 0"]
impl crate::Resettable for Rki2cMtxcntSpec {
    const RESET_VALUE: u32 = 0;
}
