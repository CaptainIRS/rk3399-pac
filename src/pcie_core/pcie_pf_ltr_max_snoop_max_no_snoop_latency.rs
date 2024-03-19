#[doc = "Register `PCIE_PF_LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY` reader"]
pub type R = crate::R<PciePfLtrMaxSnoopMaxNoSnoopLatencySpec>;
#[doc = "Register `PCIE_PF_LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY` writer"]
pub type W = crate::W<PciePfLtrMaxSnoopMaxNoSnoopLatencySpec>;
#[doc = "Field `MSL` reader - Max Snoop Latency \\[MSL\\]\n\nWhen multiplied by the value of the\n\nMax Snoop Latency Scale, this field\n\ndefines the maximum snoop value\n\nthe device is permitted to request in\n\nan LTR message. This field can be\n\nwritten independently for each\n\nPhysical Function from the local\n\nmanagement bus."]
pub type MslR = crate::FieldReader<u16>;
#[doc = "Field `MSL` writer - Max Snoop Latency \\[MSL\\]\n\nWhen multiplied by the value of the\n\nMax Snoop Latency Scale, this field\n\ndefines the maximum snoop value\n\nthe device is permitted to request in\n\nan LTR message. This field can be\n\nwritten independently for each\n\nPhysical Function from the local\n\nmanagement bus."]
pub type MslW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MSLS` reader - Max Snoop Latency Scale \\[MSLS\\]\n\nSpecifies the scale value for the Max\n\nSnoop Latency. When the setting of\n\nthis field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max Snoop Latency\n\nby the following scale factors: 001:\n\n32 ns, 010: 1024 ns, 011: 32,768\n\nns, 100: 1,047,576 ns, 101:\n\n33,554,432ns, 110-111: Reserved"]
pub type MslsR = crate::FieldReader;
#[doc = "Field `MSLS` writer - Max Snoop Latency Scale \\[MSLS\\]\n\nSpecifies the scale value for the Max\n\nSnoop Latency. When the setting of\n\nthis field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max Snoop Latency\n\nby the following scale factors: 001:\n\n32 ns, 010: 1024 ns, 011: 32,768\n\nns, 100: 1,047,576 ns, 101:\n\n33,554,432ns, 110-111: Reserved"]
pub type MslsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `MNSL` reader - Max No- Snoop Latency \\[MNSL\\]\n\nWhen multiplied by the value of the\n\nMax No- Snoop Latency Scale, this\n\nfield defines the maximum no-snoop\n\nvalue the device is permitted to\n\nrequest in an LTR message. This field\n\ncan be written independently for\n\neach Physical Function from the local\n\nmanagement bus."]
pub type MnslR = crate::FieldReader<u16>;
#[doc = "Field `MNSL` writer - Max No- Snoop Latency \\[MNSL\\]\n\nWhen multiplied by the value of the\n\nMax No- Snoop Latency Scale, this\n\nfield defines the maximum no-snoop\n\nvalue the device is permitted to\n\nrequest in an LTR message. This field\n\ncan be written independently for\n\neach Physical Function from the local\n\nmanagement bus."]
pub type MnslW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Max No- Snoop Latency Scale \\[MNSLS\\]\n\nSpecifies the scale value for the Max\n\nNo-Snoop Latency. When the setting\n\nof this field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max No-Snoop\n\nLatency by the following scale\n\nfactors: 001: 32 ns, 010: 1024 ns,\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mnsls {
    #[doc = "3: 32,768 ns, 100: 1,047,576 ns,"]
    B011 = 3,
    #[doc = "5: 33,554,432ns, 110-111: Reserved"]
    B101 = 5,
}
impl From<Mnsls> for u8 {
    #[inline(always)]
    fn from(variant: Mnsls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mnsls {
    type Ux = u8;
}
#[doc = "Field `MNSLS` reader - Max No- Snoop Latency Scale \\[MNSLS\\]\n\nSpecifies the scale value for the Max\n\nNo-Snoop Latency. When the setting\n\nof this field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max No-Snoop\n\nLatency by the following scale\n\nfactors: 001: 32 ns, 010: 1024 ns,"]
pub type MnslsR = crate::FieldReader<Mnsls>;
impl MnslsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mnsls> {
        match self.bits {
            3 => Some(Mnsls::B011),
            5 => Some(Mnsls::B101),
            _ => None,
        }
    }
    #[doc = "32,768 ns, 100: 1,047,576 ns,"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Mnsls::B011
    }
    #[doc = "33,554,432ns, 110-111: Reserved"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Mnsls::B101
    }
}
#[doc = "Field `MNSLS` writer - Max No- Snoop Latency Scale \\[MNSLS\\]\n\nSpecifies the scale value for the Max\n\nNo-Snoop Latency. When the setting\n\nof this field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max No-Snoop\n\nLatency by the following scale\n\nfactors: 001: 32 ns, 010: 1024 ns,"]
pub type MnslsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mnsls>;
impl<'a, REG> MnslsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32,768 ns, 100: 1,047,576 ns,"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Mnsls::B011)
    }
    #[doc = "33,554,432ns, 110-111: Reserved"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Mnsls::B101)
    }
}
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - Max Snoop Latency \\[MSL\\]\n\nWhen multiplied by the value of the\n\nMax Snoop Latency Scale, this field\n\ndefines the maximum snoop value\n\nthe device is permitted to request in\n\nan LTR message. This field can be\n\nwritten independently for each\n\nPhysical Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        MslR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - Max Snoop Latency Scale \\[MSLS\\]\n\nSpecifies the scale value for the Max\n\nSnoop Latency. When the setting of\n\nthis field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max Snoop Latency\n\nby the following scale factors: 001:\n\n32 ns, 010: 1024 ns, 011: 32,768\n\nns, 100: 1,047,576 ns, 101:\n\n33,554,432ns, 110-111: Reserved"]
    #[inline(always)]
    pub fn msls(&self) -> MslsR {
        MslsR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Max No- Snoop Latency \\[MNSL\\]\n\nWhen multiplied by the value of the\n\nMax No- Snoop Latency Scale, this\n\nfield defines the maximum no-snoop\n\nvalue the device is permitted to\n\nrequest in an LTR message. This field\n\ncan be written independently for\n\neach Physical Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn mnsl(&self) -> MnslR {
        MnslR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:28 - Max No- Snoop Latency Scale \\[MNSLS\\]\n\nSpecifies the scale value for the Max\n\nNo-Snoop Latency. When the setting\n\nof this field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max No-Snoop\n\nLatency by the following scale\n\nfactors: 001: 32 ns, 010: 1024 ns,"]
    #[inline(always)]
    pub fn mnsls(&self) -> MnslsR {
        MnslsR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Max Snoop Latency \\[MSL\\]\n\nWhen multiplied by the value of the\n\nMax Snoop Latency Scale, this field\n\ndefines the maximum snoop value\n\nthe device is permitted to request in\n\nan LTR message. This field can be\n\nwritten independently for each\n\nPhysical Function from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn msl(&mut self) -> MslW<PciePfLtrMaxSnoopMaxNoSnoopLatencySpec> {
        MslW::new(self, 0)
    }
    #[doc = "Bits 10:12 - Max Snoop Latency Scale \\[MSLS\\]\n\nSpecifies the scale value for the Max\n\nSnoop Latency. When the setting of\n\nthis field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max Snoop Latency\n\nby the following scale factors: 001:\n\n32 ns, 010: 1024 ns, 011: 32,768\n\nns, 100: 1,047,576 ns, 101:\n\n33,554,432ns, 110-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn msls(&mut self) -> MslsW<PciePfLtrMaxSnoopMaxNoSnoopLatencySpec> {
        MslsW::new(self, 10)
    }
    #[doc = "Bits 16:25 - Max No- Snoop Latency \\[MNSL\\]\n\nWhen multiplied by the value of the\n\nMax No- Snoop Latency Scale, this\n\nfield defines the maximum no-snoop\n\nvalue the device is permitted to\n\nrequest in an LTR message. This field\n\ncan be written independently for\n\neach Physical Function from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn mnsl(&mut self) -> MnslW<PciePfLtrMaxSnoopMaxNoSnoopLatencySpec> {
        MnslW::new(self, 16)
    }
    #[doc = "Bits 26:28 - Max No- Snoop Latency Scale \\[MNSLS\\]\n\nSpecifies the scale value for the Max\n\nNo-Snoop Latency. When the setting\n\nof this field is non- zero, the actual\n\nsnoop latency is determined by\n\nmultiplying the Max No-Snoop\n\nLatency by the following scale\n\nfactors: 001: 32 ns, 010: 1024 ns,"]
    #[inline(always)]
    #[must_use]
    pub fn mnsls(&mut self) -> MnslsW<PciePfLtrMaxSnoopMaxNoSnoopLatencySpec> {
        MnslsW::new(self, 26)
    }
}
#[doc = "LTR Max Snoop/Max No-Snoop Latency Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_ltr_max_snoop_max_no_snoop_latency::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_ltr_max_snoop_max_no_snoop_latency::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfLtrMaxSnoopMaxNoSnoopLatencySpec;
impl crate::RegisterSpec for PciePfLtrMaxSnoopMaxNoSnoopLatencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_ltr_max_snoop_max_no_snoop_latency::R`](R) reader structure"]
impl crate::Readable for PciePfLtrMaxSnoopMaxNoSnoopLatencySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_ltr_max_snoop_max_no_snoop_latency::W`](W) writer structure"]
impl crate::Writable for PciePfLtrMaxSnoopMaxNoSnoopLatencySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_LTR_MAX_SNOOP_MAX_NO_SNOOP_LATENCY to value 0"]
impl crate::Resettable for PciePfLtrMaxSnoopMaxNoSnoopLatencySpec {
    const RESET_VALUE: u32 = 0;
}
