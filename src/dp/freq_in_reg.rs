#[doc = "Register `FREQ_IN_REG` reader"]
pub type R = crate::R<FreqInRegSpec>;
#[doc = "Register `FREQ_IN_REG` writer"]
pub type W = crate::W<FreqInRegSpec>;
#[doc = "Field `FREQ_REG` reader - frequency set from register for freq counter"]
pub type FreqRegR = crate::FieldReader;
#[doc = "Field `FREQ_REG` writer - frequency set from register for freq counter"]
pub type FreqRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - frequency set from register for freq counter"]
    #[inline(always)]
    pub fn freq_reg(&self) -> FreqRegR {
        FreqRegR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - frequency set from register for freq counter"]
    #[inline(always)]
    #[must_use]
    pub fn freq_reg(&mut self) -> FreqRegW<FreqInRegSpec> {
        FreqRegW::new(self, 0)
    }
}
#[doc = "freq_in_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freq_in_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freq_in_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqInRegSpec;
impl crate::RegisterSpec for FreqInRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freq_in_reg::R`](R) reader structure"]
impl crate::Readable for FreqInRegSpec {}
#[doc = "`write(|w| ..)` method takes [`freq_in_reg::W`](W) writer structure"]
impl crate::Writable for FreqInRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQ_IN_REG to value 0x80"]
impl crate::Resettable for FreqInRegSpec {
    const RESET_VALUE: u32 = 0x80;
}
