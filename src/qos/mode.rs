#[doc = "Register `Mode` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `Mode` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `MODE` reader - 0 = Programmable mode: a programmed priority is assigned to\n\neach read or write,\n\n1 = Bandwidth Limiter Mode: a hard limit restricts throughput,\n\n2 = Bypass mode: (&lt;See SoC-specific QoS generator\n\ndocumentation>),\n\n3 = Bandwidth Regulator mode: priority decreases when\n\nthroughput exceeds a threshold."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - 0 = Programmable mode: a programmed priority is assigned to\n\neach read or write,\n\n1 = Bandwidth Limiter Mode: a hard limit restricts throughput,\n\n2 = Bypass mode: (&lt;See SoC-specific QoS generator\n\ndocumentation>),\n\n3 = Bandwidth Regulator mode: priority decreases when\n\nthroughput exceeds a threshold."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0 = Programmable mode: a programmed priority is assigned to\n\neach read or write,\n\n1 = Bandwidth Limiter Mode: a hard limit restricts throughput,\n\n2 = Bypass mode: (&lt;See SoC-specific QoS generator\n\ndocumentation>),\n\n3 = Bandwidth Regulator mode: priority decreases when\n\nthroughput exceeds a threshold."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0 = Programmable mode: a programmed priority is assigned to\n\neach read or write,\n\n1 = Bandwidth Limiter Mode: a hard limit restricts throughput,\n\n2 = Bypass mode: (&lt;See SoC-specific QoS generator\n\ndocumentation>),\n\n3 = Bandwidth Regulator mode: priority decreases when\n\nthroughput exceeds a threshold."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ModeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Mode to value 0x03"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x03;
}
