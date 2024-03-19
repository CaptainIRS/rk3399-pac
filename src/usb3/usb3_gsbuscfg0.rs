#[doc = "Register `USB3_GSBUSCFG0` reader"]
pub type R = crate::R<Usb3Gsbuscfg0Spec>;
#[doc = "Register `USB3_GSBUSCFG0` writer"]
pub type W = crate::W<Usb3Gsbuscfg0Spec>;
#[doc = "Undefined Length INCR Burst Type Enable\n\nThis bit determines the set of burst lengths the master interface\n\nuses. It works in conjunction with the\n\nGSBUSCFG0\\[7:1\\]
enables (INCR256/128/64/32/16/8/4).\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incrbrstena {
    #[doc = "0: INCRX burst mode ARLEN/AWLEN do not use INCR. They use only the following burst lengths: 1; 4 (if GSBUSCFG0.INCR4BrstEna = 1); 8 (if GSBUSCFG0.INCR8BrstEna = 1); 16 (if GSBUSCFG0.INCR16BrstEna = 1); 32 (if GSBUSCFG0.INCR32BrstEna = 1); 64 (if GSBUSCFG0.INCR64BrstEna = 1); 128 (if GSBUSCFG0.INCR128BrstEna = 1); 256 (if GSBUSCFG0.INCR256BrstEna = 1);"]
    B0 = 0,
    #[doc = "1: INCR (undefined length) burst mode ARLEN/AWLEN uses any length less than or equal to the largest- enabled burst length of INCR4/8/16/32/64/128/256. For cache line-aligned applications, this bit is typically set to 0 to ensure that the master interface uses only power-of-2 burst lengths (as enabled via GSBUSCFG0\\[7:0\\])."]
    B1 = 1,
}
impl From<Incrbrstena> for bool {
    #[inline(always)]
    fn from(variant: Incrbrstena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCRBRSTENA` reader - Undefined Length INCR Burst Type Enable\n\nThis bit determines the set of burst lengths the master interface\n\nuses. It works in conjunction with the\n\nGSBUSCFG0\\[7:1\\]
enables (INCR256/128/64/32/16/8/4)."]
pub type IncrbrstenaR = crate::BitReader<Incrbrstena>;
impl IncrbrstenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Incrbrstena {
        match self.bits {
            false => Incrbrstena::B0,
            true => Incrbrstena::B1,
        }
    }
    #[doc = "INCRX burst mode ARLEN/AWLEN do not use INCR. They use only the following burst lengths: 1; 4 (if GSBUSCFG0.INCR4BrstEna = 1); 8 (if GSBUSCFG0.INCR8BrstEna = 1); 16 (if GSBUSCFG0.INCR16BrstEna = 1); 32 (if GSBUSCFG0.INCR32BrstEna = 1); 64 (if GSBUSCFG0.INCR64BrstEna = 1); 128 (if GSBUSCFG0.INCR128BrstEna = 1); 256 (if GSBUSCFG0.INCR256BrstEna = 1);"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Incrbrstena::B0
    }
    #[doc = "INCR (undefined length) burst mode ARLEN/AWLEN uses any length less than or equal to the largest- enabled burst length of INCR4/8/16/32/64/128/256. For cache line-aligned applications, this bit is typically set to 0 to ensure that the master interface uses only power-of-2 burst lengths (as enabled via GSBUSCFG0\\[7:0\\])."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Incrbrstena::B1
    }
}
#[doc = "Field `INCRBRSTENA` writer - Undefined Length INCR Burst Type Enable\n\nThis bit determines the set of burst lengths the master interface\n\nuses. It works in conjunction with the\n\nGSBUSCFG0\\[7:1\\]
enables (INCR256/128/64/32/16/8/4)."]
pub type IncrbrstenaW<'a, REG> = crate::BitWriter<'a, REG, Incrbrstena>;
impl<'a, REG> IncrbrstenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INCRX burst mode ARLEN/AWLEN do not use INCR. They use only the following burst lengths: 1; 4 (if GSBUSCFG0.INCR4BrstEna = 1); 8 (if GSBUSCFG0.INCR8BrstEna = 1); 16 (if GSBUSCFG0.INCR16BrstEna = 1); 32 (if GSBUSCFG0.INCR32BrstEna = 1); 64 (if GSBUSCFG0.INCR64BrstEna = 1); 128 (if GSBUSCFG0.INCR128BrstEna = 1); 256 (if GSBUSCFG0.INCR256BrstEna = 1);"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Incrbrstena::B0)
    }
    #[doc = "INCR (undefined length) burst mode ARLEN/AWLEN uses any length less than or equal to the largest- enabled burst length of INCR4/8/16/32/64/128/256. For cache line-aligned applications, this bit is typically set to 0 to ensure that the master interface uses only power-of-2 burst lengths (as enabled via GSBUSCFG0\\[7:0\\])."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Incrbrstena::B1)
    }
}
#[doc = "Field `INCR4BRSTENA` reader - INCR4 Burst Type Enable\n\nWhen this bit is enabled the controller is allowed to do bursts of\n\nbeat length 1, 2, 3, and 4. It is highly recommended that this bit\n\nis enabled to prevent descriptor reads and writes from being\n\nbroken up into separate transfers."]
pub type Incr4brstenaR = crate::BitReader;
#[doc = "Field `INCR4BRSTENA` writer - INCR4 Burst Type Enable\n\nWhen this bit is enabled the controller is allowed to do bursts of\n\nbeat length 1, 2, 3, and 4. It is highly recommended that this bit\n\nis enabled to prevent descriptor reads and writes from being\n\nbroken up into separate transfers."]
pub type Incr4brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR8BRSTENA` reader - INCR8 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n8-beat burst."]
pub type Incr8brstenaR = crate::BitReader;
#[doc = "Field `INCR8BRSTENA` writer - INCR8 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n8-beat burst."]
pub type Incr8brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR16BRSTENA` reader - INCR16 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n16-beat burst."]
pub type Incr16brstenaR = crate::BitReader;
#[doc = "Field `INCR16BRSTENA` writer - INCR16 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n16-beat burst."]
pub type Incr16brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR32BRSTENA` reader - INCR32 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n32-beat burst."]
pub type Incr32brstenaR = crate::BitReader;
#[doc = "Field `INCR32BRSTENA` writer - INCR32 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n32-beat burst."]
pub type Incr32brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR64BRSTENA` reader - INCR64 Burst Type Enable\n\nIf software set this bit to 1, AXI master uses INCR to do the 64-\n\nbeat burst."]
pub type Incr64brstenaR = crate::BitReader;
#[doc = "Field `INCR64BRSTENA` writer - INCR64 Burst Type Enable\n\nIf software set this bit to 1, AXI master uses INCR to do the 64-\n\nbeat burst."]
pub type Incr64brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR128BRSTENA` reader - INCR128 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n128-beat burst."]
pub type Incr128brstenaR = crate::BitReader;
#[doc = "Field `INCR128BRSTENA` writer - INCR128 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n128-beat burst."]
pub type Incr128brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR256BRSTENA` reader - INCR256 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n256-beat burst."]
pub type Incr256brstenaR = crate::BitReader;
#[doc = "Field `INCR256BRSTENA` writer - INCR256 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n256-beat burst."]
pub type Incr256brstenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESBIGEND` reader - Descriptor Access is Big Endian\n\nThis bit controls the endian mode for descriptor accesses. 0,\n\nLittle-endian (default); 1, Big-endian."]
pub type DesbigendR = crate::BitReader;
#[doc = "Field `DESBIGEND` writer - Descriptor Access is Big Endian\n\nThis bit controls the endian mode for descriptor accesses. 0,\n\nLittle-endian (default); 1, Big-endian."]
pub type DesbigendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATBIGEND` reader - Data Access is Big Endian\n\nThis bit controls the endian mode for data accesses.0, Little-\n\nendian (default); 1, Big-endian;"]
pub type DatbigendR = crate::BitReader;
#[doc = "Field `DATBIGEND` writer - Data Access is Big Endian\n\nThis bit controls the endian mode for data accesses.0, Little-\n\nendian (default); 1, Big-endian;"]
pub type DatbigendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESWRREQINFO` reader - DESWRREQINFO\n\nAXI-cache for Descriptor Write (DesWrReqInfo)"]
pub type DeswrreqinfoR = crate::FieldReader;
#[doc = "Field `DESWRREQINFO` writer - DESWRREQINFO\n\nAXI-cache for Descriptor Write (DesWrReqInfo)"]
pub type DeswrreqinfoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATWRREQINFO` reader - DATWRREQINFO\n\nAXI-cache for Data Write (DatWrReqInfo)."]
pub type DatwrreqinfoR = crate::FieldReader;
#[doc = "Field `DATWRREQINFO` writer - DATWRREQINFO\n\nAXI-cache for Data Write (DatWrReqInfo)."]
pub type DatwrreqinfoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DESRDREQINFO` reader - DESRDREQINFO\n\nAXI-cache for Descriptor Read (DesRdReqInfo)."]
pub type DesrdreqinfoR = crate::FieldReader;
#[doc = "Field `DESRDREQINFO` writer - DESRDREQINFO\n\nAXI-cache for Descriptor Read (DesRdReqInfo)."]
pub type DesrdreqinfoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATRDREQINFO` reader - DATRDREQINFO\n\nAXI-cache for Data Read (DatRdReqInfo)"]
pub type DatrdreqinfoR = crate::FieldReader;
#[doc = "Field `DATRDREQINFO` writer - DATRDREQINFO\n\nAXI-cache for Data Read (DatRdReqInfo)"]
pub type DatrdreqinfoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Undefined Length INCR Burst Type Enable\n\nThis bit determines the set of burst lengths the master interface\n\nuses. It works in conjunction with the\n\nGSBUSCFG0\\[7:1\\]
enables (INCR256/128/64/32/16/8/4)."]
    #[inline(always)]
    pub fn incrbrstena(&self) -> IncrbrstenaR {
        IncrbrstenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Type Enable\n\nWhen this bit is enabled the controller is allowed to do bursts of\n\nbeat length 1, 2, 3, and 4. It is highly recommended that this bit\n\nis enabled to prevent descriptor reads and writes from being\n\nbroken up into separate transfers."]
    #[inline(always)]
    pub fn incr4brstena(&self) -> Incr4brstenaR {
        Incr4brstenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INCR8 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n8-beat burst."]
    #[inline(always)]
    pub fn incr8brstena(&self) -> Incr8brstenaR {
        Incr8brstenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INCR16 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n16-beat burst."]
    #[inline(always)]
    pub fn incr16brstena(&self) -> Incr16brstenaR {
        Incr16brstenaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INCR32 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n32-beat burst."]
    #[inline(always)]
    pub fn incr32brstena(&self) -> Incr32brstenaR {
        Incr32brstenaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INCR64 Burst Type Enable\n\nIf software set this bit to 1, AXI master uses INCR to do the 64-\n\nbeat burst."]
    #[inline(always)]
    pub fn incr64brstena(&self) -> Incr64brstenaR {
        Incr64brstenaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INCR128 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n128-beat burst."]
    #[inline(always)]
    pub fn incr128brstena(&self) -> Incr128brstenaR {
        Incr128brstenaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INCR256 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n256-beat burst."]
    #[inline(always)]
    pub fn incr256brstena(&self) -> Incr256brstenaR {
        Incr256brstenaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Descriptor Access is Big Endian\n\nThis bit controls the endian mode for descriptor accesses. 0,\n\nLittle-endian (default); 1, Big-endian."]
    #[inline(always)]
    pub fn desbigend(&self) -> DesbigendR {
        DesbigendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Access is Big Endian\n\nThis bit controls the endian mode for data accesses.0, Little-\n\nendian (default); 1, Big-endian;"]
    #[inline(always)]
    pub fn datbigend(&self) -> DatbigendR {
        DatbigendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DESWRREQINFO\n\nAXI-cache for Descriptor Write (DesWrReqInfo)"]
    #[inline(always)]
    pub fn deswrreqinfo(&self) -> DeswrreqinfoR {
        DeswrreqinfoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DATWRREQINFO\n\nAXI-cache for Data Write (DatWrReqInfo)."]
    #[inline(always)]
    pub fn datwrreqinfo(&self) -> DatwrreqinfoR {
        DatwrreqinfoR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DESRDREQINFO\n\nAXI-cache for Descriptor Read (DesRdReqInfo)."]
    #[inline(always)]
    pub fn desrdreqinfo(&self) -> DesrdreqinfoR {
        DesrdreqinfoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DATRDREQINFO\n\nAXI-cache for Data Read (DatRdReqInfo)"]
    #[inline(always)]
    pub fn datrdreqinfo(&self) -> DatrdreqinfoR {
        DatrdreqinfoR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Undefined Length INCR Burst Type Enable\n\nThis bit determines the set of burst lengths the master interface\n\nuses. It works in conjunction with the\n\nGSBUSCFG0\\[7:1\\]
enables (INCR256/128/64/32/16/8/4)."]
    #[inline(always)]
    #[must_use]
    pub fn incrbrstena(&mut self) -> IncrbrstenaW<Usb3Gsbuscfg0Spec> {
        IncrbrstenaW::new(self, 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Type Enable\n\nWhen this bit is enabled the controller is allowed to do bursts of\n\nbeat length 1, 2, 3, and 4. It is highly recommended that this bit\n\nis enabled to prevent descriptor reads and writes from being\n\nbroken up into separate transfers."]
    #[inline(always)]
    #[must_use]
    pub fn incr4brstena(&mut self) -> Incr4brstenaW<Usb3Gsbuscfg0Spec> {
        Incr4brstenaW::new(self, 1)
    }
    #[doc = "Bit 2 - INCR8 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n8-beat burst."]
    #[inline(always)]
    #[must_use]
    pub fn incr8brstena(&mut self) -> Incr8brstenaW<Usb3Gsbuscfg0Spec> {
        Incr8brstenaW::new(self, 2)
    }
    #[doc = "Bit 3 - INCR16 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n16-beat burst."]
    #[inline(always)]
    #[must_use]
    pub fn incr16brstena(&mut self) -> Incr16brstenaW<Usb3Gsbuscfg0Spec> {
        Incr16brstenaW::new(self, 3)
    }
    #[doc = "Bit 4 - INCR32 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n32-beat burst."]
    #[inline(always)]
    #[must_use]
    pub fn incr32brstena(&mut self) -> Incr32brstenaW<Usb3Gsbuscfg0Spec> {
        Incr32brstenaW::new(self, 4)
    }
    #[doc = "Bit 5 - INCR64 Burst Type Enable\n\nIf software set this bit to 1, AXI master uses INCR to do the 64-\n\nbeat burst."]
    #[inline(always)]
    #[must_use]
    pub fn incr64brstena(&mut self) -> Incr64brstenaW<Usb3Gsbuscfg0Spec> {
        Incr64brstenaW::new(self, 5)
    }
    #[doc = "Bit 6 - INCR128 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n128-beat burst."]
    #[inline(always)]
    #[must_use]
    pub fn incr128brstena(&mut self) -> Incr128brstenaW<Usb3Gsbuscfg0Spec> {
        Incr128brstenaW::new(self, 6)
    }
    #[doc = "Bit 7 - INCR256 Burst Type Enable\n\nIf software set this bit to 1, the AXI master uses INCR to do the\n\n256-beat burst."]
    #[inline(always)]
    #[must_use]
    pub fn incr256brstena(&mut self) -> Incr256brstenaW<Usb3Gsbuscfg0Spec> {
        Incr256brstenaW::new(self, 7)
    }
    #[doc = "Bit 10 - Descriptor Access is Big Endian\n\nThis bit controls the endian mode for descriptor accesses. 0,\n\nLittle-endian (default); 1, Big-endian."]
    #[inline(always)]
    #[must_use]
    pub fn desbigend(&mut self) -> DesbigendW<Usb3Gsbuscfg0Spec> {
        DesbigendW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Access is Big Endian\n\nThis bit controls the endian mode for data accesses.0, Little-\n\nendian (default); 1, Big-endian;"]
    #[inline(always)]
    #[must_use]
    pub fn datbigend(&mut self) -> DatbigendW<Usb3Gsbuscfg0Spec> {
        DatbigendW::new(self, 11)
    }
    #[doc = "Bits 16:19 - DESWRREQINFO\n\nAXI-cache for Descriptor Write (DesWrReqInfo)"]
    #[inline(always)]
    #[must_use]
    pub fn deswrreqinfo(&mut self) -> DeswrreqinfoW<Usb3Gsbuscfg0Spec> {
        DeswrreqinfoW::new(self, 16)
    }
    #[doc = "Bits 20:23 - DATWRREQINFO\n\nAXI-cache for Data Write (DatWrReqInfo)."]
    #[inline(always)]
    #[must_use]
    pub fn datwrreqinfo(&mut self) -> DatwrreqinfoW<Usb3Gsbuscfg0Spec> {
        DatwrreqinfoW::new(self, 20)
    }
    #[doc = "Bits 24:27 - DESRDREQINFO\n\nAXI-cache for Descriptor Read (DesRdReqInfo)."]
    #[inline(always)]
    #[must_use]
    pub fn desrdreqinfo(&mut self) -> DesrdreqinfoW<Usb3Gsbuscfg0Spec> {
        DesrdreqinfoW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DATRDREQINFO\n\nAXI-cache for Data Read (DatRdReqInfo)"]
    #[inline(always)]
    #[must_use]
    pub fn datrdreqinfo(&mut self) -> DatrdreqinfoW<Usb3Gsbuscfg0Spec> {
        DatrdreqinfoW::new(self, 28)
    }
}
#[doc = "Global SoC Bus Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsbuscfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gsbuscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Gsbuscfg0Spec;
impl crate::RegisterSpec for Usb3Gsbuscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gsbuscfg0::R`](R) reader structure"]
impl crate::Readable for Usb3Gsbuscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gsbuscfg0::W`](W) writer structure"]
impl crate::Writable for Usb3Gsbuscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GSBUSCFG0 to value 0x01"]
impl crate::Resettable for Usb3Gsbuscfg0Spec {
    const RESET_VALUE: u32 = 0x01;
}
