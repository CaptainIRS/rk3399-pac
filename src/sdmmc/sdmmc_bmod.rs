#[doc = "Register `SDMMC_BMOD` reader"]
pub type R = crate::R<SdmmcBmodSpec>;
#[doc = "Register `SDMMC_BMOD` writer"]
pub type W = crate::W<SdmmcBmodSpec>;
#[doc = "Field `SWR` reader - Software Reset. When set, the DMA Controller resets all its\n\ninternal registers.\n\nIt is automatically cleared after 1 clock cycle."]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset. When set, the DMA Controller resets all its\n\ninternal registers.\n\nIt is automatically cleared after 1 clock cycle."]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB` reader - Fixed Burst. Controls whether the AHB Master interface performs\n\nfixed burst transfers or not. When set, the AHB will use only\n\nSINGLE, INCR4, INCR8 or INCR16 during start of normal burst\n\ntransfers. When reset, the AHB will use SINGLE and INCR burst\n\ntransfer operations."]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst. Controls whether the AHB Master interface performs\n\nfixed burst transfers or not. When set, the AHB will use only\n\nSINGLE, INCR4, INCR8 or INCR16 during start of normal burst\n\ntransfers. When reset, the AHB will use SINGLE and INCR burst\n\ntransfer operations."]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length. Specifies the number of\n\nHWord/Word/Dword (depending on 16/32/64-bit bus) to skip\n\nbetween two unchained descriptors. This is applicable only for\n\ndual buffer structure."]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length. Specifies the number of\n\nHWord/Word/Dword (depending on 16/32/64-bit bus) to skip\n\nbetween two unchained descriptors. This is applicable only for\n\ndual buffer structure."]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DE` reader - IDMAC Enable. When set, the IDMAC is enabled."]
pub type DeR = crate::BitReader;
#[doc = "Field `DE` writer - IDMAC Enable. When set, the IDMAC is enabled."]
pub type DeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Programmable Burst Length. These bits indicate the maximum\n\nnumber of beats to be performed in one IDMAC transaction. The\n\nIDMAC will always attempt to burst as specified in PBL each time\n\nit starts a Burst transfer on the host bus. The permissible values\n\nare 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of\n\nMSIZE of FIFOTH register. In order to change this value, write\n\nthe required value to FIFOTH register. This is an encode value as\n\nfollows.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbl {
    #[doc = "0: 1 transfers"]
    D0 = 0,
    #[doc = "1: 4 transfers"]
    D1 = 1,
    #[doc = "2: 8 transfers"]
    D2 = 2,
    #[doc = "3: 16 transfers"]
    D3 = 3,
    #[doc = "4: 32 transfers"]
    D4 = 4,
    #[doc = "5: 64 transfers"]
    D5 = 5,
    #[doc = "6: 128 transfers"]
    D6 = 6,
    #[doc = "7: 256 transfers Transfer unit is either 16, 32, or 64 bits, based on HDATA_WIDTH. PBL is a read-only value and is applicable only for Data Access; it does not apply to descriptor accesses."]
    D7 = 7,
}
impl From<Pbl> for u8 {
    #[inline(always)]
    fn from(variant: Pbl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pbl {
    type Ux = u8;
}
#[doc = "Field `PBL` reader - Programmable Burst Length. These bits indicate the maximum\n\nnumber of beats to be performed in one IDMAC transaction. The\n\nIDMAC will always attempt to burst as specified in PBL each time\n\nit starts a Burst transfer on the host bus. The permissible values\n\nare 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of\n\nMSIZE of FIFOTH register. In order to change this value, write\n\nthe required value to FIFOTH register. This is an encode value as\n\nfollows."]
pub type PblR = crate::FieldReader<Pbl>;
impl PblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbl {
        match self.bits {
            0 => Pbl::D0,
            1 => Pbl::D1,
            2 => Pbl::D2,
            3 => Pbl::D3,
            4 => Pbl::D4,
            5 => Pbl::D5,
            6 => Pbl::D6,
            7 => Pbl::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "1 transfers"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Pbl::D0
    }
    #[doc = "4 transfers"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Pbl::D1
    }
    #[doc = "8 transfers"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Pbl::D2
    }
    #[doc = "16 transfers"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Pbl::D3
    }
    #[doc = "32 transfers"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == Pbl::D4
    }
    #[doc = "64 transfers"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == Pbl::D5
    }
    #[doc = "128 transfers"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == Pbl::D6
    }
    #[doc = "256 transfers Transfer unit is either 16, 32, or 64 bits, based on HDATA_WIDTH. PBL is a read-only value and is applicable only for Data Access; it does not apply to descriptor accesses."]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == Pbl::D7
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its\n\ninternal registers.\n\nIt is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs\n\nfixed burst transfers or not. When set, the AHB will use only\n\nSINGLE, INCR4, INCR8 or INCR16 during start of normal burst\n\ntransfers. When reset, the AHB will use SINGLE and INCR burst\n\ntransfer operations."]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length. Specifies the number of\n\nHWord/Word/Dword (depending on 16/32/64-bit bus) to skip\n\nbetween two unchained descriptors. This is applicable only for\n\ndual buffer structure."]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - IDMAC Enable. When set, the IDMAC is enabled."]
    #[inline(always)]
    pub fn de(&self) -> DeR {
        DeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length. These bits indicate the maximum\n\nnumber of beats to be performed in one IDMAC transaction. The\n\nIDMAC will always attempt to burst as specified in PBL each time\n\nit starts a Burst transfer on the host bus. The permissible values\n\nare 1, 4, 8, 16, 32, 64, 128 and 256. This value is the mirror of\n\nMSIZE of FIFOTH register. In order to change this value, write\n\nthe required value to FIFOTH register. This is an encode value as\n\nfollows."]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset. When set, the DMA Controller resets all its\n\ninternal registers.\n\nIt is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SwrW<SdmmcBmodSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 1 - Fixed Burst. Controls whether the AHB Master interface performs\n\nfixed burst transfers or not. When set, the AHB will use only\n\nSINGLE, INCR4, INCR8 or INCR16 during start of normal burst\n\ntransfers. When reset, the AHB will use SINGLE and INCR burst\n\ntransfer operations."]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FbW<SdmmcBmodSpec> {
        FbW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length. Specifies the number of\n\nHWord/Word/Dword (depending on 16/32/64-bit bus) to skip\n\nbetween two unchained descriptors. This is applicable only for\n\ndual buffer structure."]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DslW<SdmmcBmodSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bit 7 - IDMAC Enable. When set, the IDMAC is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DeW<SdmmcBmodSpec> {
        DeW::new(self, 7)
    }
}
#[doc = "Bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_bmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_bmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcBmodSpec;
impl crate::RegisterSpec for SdmmcBmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_bmod::R`](R) reader structure"]
impl crate::Readable for SdmmcBmodSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_bmod::W`](W) writer structure"]
impl crate::Writable for SdmmcBmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_BMOD to value 0"]
impl crate::Resettable for SdmmcBmodSpec {
    const RESET_VALUE: u32 = 0;
}
