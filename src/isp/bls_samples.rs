#[doc = "Register `BLS_SAMPLES` reader"]
pub type R = crate::R<BlsSamplesSpec>;
#[doc = "Register `BLS_SAMPLES` writer"]
pub type W = crate::W<BlsSamplesSpec>;
#[doc = "Field `BLS_SAMPLES` reader - This number to the power of two gives the number of\n\nmeasure samples for each Bayer position. Range 0x00:\n\n2^0=1 to 0x12: 2^18=262144. This number is also the\n\ndivider for the accumulator for each Bayer position.\n\nThe accumulation will be stopped, if the number of\n\nmeasured pixels for the current Bayer position is equal to\n\nthe number of samples.\n\nThe measure windows must be positioned that way\n\nthat the number of included pixels of each Bayer position\n\nincluded by both windows is equal or greater than the\n\nnumber of measure samples calculated by 2^BLS_SAMPLES !\n\nNOTE: The number of pixels of one Bayer position is\n\n1/4 of the number of all Pixels included by the measure\n\nwindows.\n\n"]
pub type BlsSamplesR = crate::FieldReader;
#[doc = "Field `BLS_SAMPLES` writer - This number to the power of two gives the number of\n\nmeasure samples for each Bayer position. Range 0x00:\n\n2^0=1 to 0x12: 2^18=262144. This number is also the\n\ndivider for the accumulator for each Bayer position.\n\nThe accumulation will be stopped, if the number of\n\nmeasured pixels for the current Bayer position is equal to\n\nthe number of samples.\n\nThe measure windows must be positioned that way\n\nthat the number of included pixels of each Bayer position\n\nincluded by both windows is equal or greater than the\n\nnumber of measure samples calculated by 2^BLS_SAMPLES !\n\nNOTE: The number of pixels of one Bayer position is\n\n1/4 of the number of all Pixels included by the measure\n\nwindows.\n\n"]
pub type BlsSamplesW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This number to the power of two gives the number of\n\nmeasure samples for each Bayer position. Range 0x00:\n\n2^0=1 to 0x12: 2^18=262144. This number is also the\n\ndivider for the accumulator for each Bayer position.\n\nThe accumulation will be stopped, if the number of\n\nmeasured pixels for the current Bayer position is equal to\n\nthe number of samples.\n\nThe measure windows must be positioned that way\n\nthat the number of included pixels of each Bayer position\n\nincluded by both windows is equal or greater than the\n\nnumber of measure samples calculated by 2^BLS_SAMPLES !\n\nNOTE: The number of pixels of one Bayer position is\n\n1/4 of the number of all Pixels included by the measure\n\nwindows.\n\n"]
    #[inline(always)]
    pub fn bls_samples(&self) -> BlsSamplesR {
        BlsSamplesR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This number to the power of two gives the number of\n\nmeasure samples for each Bayer position. Range 0x00:\n\n2^0=1 to 0x12: 2^18=262144. This number is also the\n\ndivider for the accumulator for each Bayer position.\n\nThe accumulation will be stopped, if the number of\n\nmeasured pixels for the current Bayer position is equal to\n\nthe number of samples.\n\nThe measure windows must be positioned that way\n\nthat the number of included pixels of each Bayer position\n\nincluded by both windows is equal or greater than the\n\nnumber of measure samples calculated by 2^BLS_SAMPLES !\n\nNOTE: The number of pixels of one Bayer position is\n\n1/4 of the number of all Pixels included by the measure\n\nwindows.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_samples(&mut self) -> BlsSamplesW<BlsSamplesSpec> {
        BlsSamplesW::new(self, 0)
    }
}
#[doc = "samples register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_samples::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_samples::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsSamplesSpec;
impl crate::RegisterSpec for BlsSamplesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_samples::R`](R) reader structure"]
impl crate::Readable for BlsSamplesSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_samples::W`](W) writer structure"]
impl crate::Writable for BlsSamplesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_SAMPLES to value 0"]
impl crate::Resettable for BlsSamplesSpec {
    const RESET_VALUE: u32 = 0;
}
