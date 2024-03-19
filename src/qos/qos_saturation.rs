#[doc = "Register `QOS_Saturation` reader"]
pub type R = crate::R<QosSaturationSpec>;
#[doc = "Register `QOS_Saturation` writer"]
pub type W = crate::W<QosSaturationSpec>;
#[doc = "Field `SATURATION` reader - In Bandwidth Limiter or Bandwidth Regulator mode, the maximum\n\ndata count value, in units of 16 bytes. This determines the window\n\nof time over which bandwidth is measured. For example, to\n\nmeasure bandwidth within a 1000 cycle window on a 64-bit\n\ninterface is value 0x1F4."]
pub type SaturationR = crate::FieldReader<u16>;
#[doc = "Field `SATURATION` writer - In Bandwidth Limiter or Bandwidth Regulator mode, the maximum\n\ndata count value, in units of 16 bytes. This determines the window\n\nof time over which bandwidth is measured. For example, to\n\nmeasure bandwidth within a 1000 cycle window on a 64-bit\n\ninterface is value 0x1F4."]
pub type SaturationW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - In Bandwidth Limiter or Bandwidth Regulator mode, the maximum\n\ndata count value, in units of 16 bytes. This determines the window\n\nof time over which bandwidth is measured. For example, to\n\nmeasure bandwidth within a 1000 cycle window on a 64-bit\n\ninterface is value 0x1F4."]
    #[inline(always)]
    pub fn saturation(&self) -> SaturationR {
        SaturationR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - In Bandwidth Limiter or Bandwidth Regulator mode, the maximum\n\ndata count value, in units of 16 bytes. This determines the window\n\nof time over which bandwidth is measured. For example, to\n\nmeasure bandwidth within a 1000 cycle window on a 64-bit\n\ninterface is value 0x1F4."]
    #[inline(always)]
    #[must_use]
    pub fn saturation(&mut self) -> SaturationW<QosSaturationSpec> {
        SaturationW::new(self, 0)
    }
}
#[doc = "Saturation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_saturation::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_saturation::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosSaturationSpec;
impl crate::RegisterSpec for QosSaturationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_saturation::R`](R) reader structure"]
impl crate::Readable for QosSaturationSpec {}
#[doc = "`write(|w| ..)` method takes [`qos_saturation::W`](W) writer structure"]
impl crate::Writable for QosSaturationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QOS_Saturation to value 0x40"]
impl crate::Resettable for QosSaturationSpec {
    const RESET_VALUE: u32 = 0x40;
}
