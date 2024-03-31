#[doc = "Register `MRXCNT` reader"]
pub type R = crate::R<MrxcntSpec>;
#[doc = "Register `MRXCNT` writer"]
pub type W = crate::W<MrxcntSpec>;
#[doc = "Field `MRXCNT` reader - master rx count\n\n6 bits counter"]
pub type MrxcntR = crate::FieldReader;
#[doc = "Field `MRXCNT` writer - master rx count\n\n6 bits counter"]
pub type MrxcntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - master rx count\n\n6 bits counter"]
    #[inline(always)]
    pub fn mrxcnt(&self) -> MrxcntR {
        MrxcntR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - master rx count\n\n6 bits counter"]
    #[inline(always)]
    #[must_use]
    pub fn mrxcnt(&mut self) -> MrxcntW<MrxcntSpec> {
        MrxcntW::new(self, 0)
    }
}
#[doc = "master rx count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrxcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrxcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrxcntSpec;
impl crate::RegisterSpec for MrxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrxcnt::R`](R) reader structure"]
impl crate::Readable for MrxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`mrxcnt::W`](W) writer structure"]
impl crate::Writable for MrxcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRXCNT to value 0"]
impl crate::Resettable for MrxcntSpec {
    const RESET_VALUE: u32 = 0;
}
