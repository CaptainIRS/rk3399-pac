#[doc = "Register `QOS_Mode` reader"]
pub type R = crate::R<QosModeSpec>;
#[doc = "Register `QOS_Mode` writer"]
pub type W = crate::W<QosModeSpec>;
#[doc = "Field `MODE` reader - 0 = Programmable mode: a programmed priority is assigned to each read or write, 1 = Bandwidth Limiter Mode: a hard limit restricts throughput, 2 = Bypass mode: (&lt;See SoC-specific QoS generator documentation>), 3 = Bandwidth Regulator mode: priority decreases when throughput exceeds a threshold."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - 0 = Programmable mode: a programmed priority is assigned to each read or write, 1 = Bandwidth Limiter Mode: a hard limit restricts throughput, 2 = Bypass mode: (&lt;See SoC-specific QoS generator documentation>), 3 = Bandwidth Regulator mode: priority decreases when throughput exceeds a threshold."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0 = Programmable mode: a programmed priority is assigned to each read or write, 1 = Bandwidth Limiter Mode: a hard limit restricts throughput, 2 = Bypass mode: (&lt;See SoC-specific QoS generator documentation>), 3 = Bandwidth Regulator mode: priority decreases when throughput exceeds a threshold."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0 = Programmable mode: a programmed priority is assigned to each read or write, 1 = Bandwidth Limiter Mode: a hard limit restricts throughput, 2 = Bypass mode: (&lt;See SoC-specific QoS generator documentation>), 3 = Bandwidth Regulator mode: priority decreases when throughput exceeds a threshold."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<QosModeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosModeSpec;
impl crate::RegisterSpec for QosModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_mode::R`](R) reader structure"]
impl crate::Readable for QosModeSpec {}
#[doc = "`write(|w| ..)` method takes [`qos_mode::W`](W) writer structure"]
impl crate::Writable for QosModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QOS_Mode to value 0x03"]
impl crate::Resettable for QosModeSpec {
    const RESET_VALUE: u32 = 0x03;
}
