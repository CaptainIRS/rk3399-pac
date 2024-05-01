#[doc = "Register `HASH_CTRL` reader"]
pub type R = crate::R<HashCtrlSpec>;
#[doc = "Register `HASH_CTRL` writer"]
pub type W = crate::W<HashCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EngineSelection {
    #[doc = "0: SHA1_HASH"]
    B00 = 0,
    #[doc = "1: MD5_HASH"]
    B01 = 1,
    #[doc = "2: SHA256_HASH"]
    B10 = 2,
    #[doc = "3: PRNG"]
    B11 = 3,
}
impl From<EngineSelection> for u8 {
    #[inline(always)]
    fn from(variant: EngineSelection) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EngineSelection {
    type Ux = u8;
}
#[doc = "Field `ENGINE_SELECTION` reader - "]
pub type EngineSelectionR = crate::FieldReader<EngineSelection>;
impl EngineSelectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EngineSelection {
        match self.bits {
            0 => EngineSelection::B00,
            1 => EngineSelection::B01,
            2 => EngineSelection::B10,
            3 => EngineSelection::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "SHA1_HASH"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == EngineSelection::B00
    }
    #[doc = "MD5_HASH"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == EngineSelection::B01
    }
    #[doc = "SHA256_HASH"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == EngineSelection::B10
    }
    #[doc = "PRNG"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == EngineSelection::B11
    }
}
#[doc = "Field `ENGINE_SELECTION` writer - "]
pub type EngineSelectionW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EngineSelection>;
impl<'a, REG> EngineSelectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHA1_HASH"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(EngineSelection::B00)
    }
    #[doc = "MD5_HASH"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(EngineSelection::B01)
    }
    #[doc = "SHA256_HASH"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(EngineSelection::B10)
    }
    #[doc = "PRNG"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(EngineSelection::B11)
    }
}
#[doc = "Field `HASH_SWAP_DO` reader - Specifies the Byte swap of data output (hash result)\n\n0 = Does not swap (default)\n\n1 = Swap"]
pub type HashSwapDoR = crate::BitReader;
#[doc = "Field `HASH_SWAP_DO` writer - Specifies the Byte swap of data output (hash result)\n\n0 = Does not swap (default)\n\n1 = Swap"]
pub type HashSwapDoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn engine_selection(&self) -> EngineSelectionR {
        EngineSelectionR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Specifies the Byte swap of data output (hash result)\n\n0 = Does not swap (default)\n\n1 = Swap"]
    #[inline(always)]
    pub fn hash_swap_do(&self) -> HashSwapDoR {
        HashSwapDoR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn engine_selection(&mut self) -> EngineSelectionW<HashCtrlSpec> {
        EngineSelectionW::new(self, 0)
    }
    #[doc = "Bit 3 - Specifies the Byte swap of data output (hash result)\n\n0 = Does not swap (default)\n\n1 = Swap"]
    #[inline(always)]
    #[must_use]
    pub fn hash_swap_do(&mut self) -> HashSwapDoW<HashCtrlSpec> {
        HashSwapDoW::new(self, 3)
    }
}
#[doc = "Hash Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashCtrlSpec;
impl crate::RegisterSpec for HashCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_ctrl::R`](R) reader structure"]
impl crate::Readable for HashCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_ctrl::W`](W) writer structure"]
impl crate::Writable for HashCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CTRL to value 0"]
impl crate::Resettable for HashCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
