#[doc = "Register `PKA_CTRL` reader"]
pub type R = crate::R<PkaCtrlSpec>;
#[doc = "Register `PKA_CTRL` writer"]
pub type W = crate::W<PkaCtrlSpec>;
#[doc = "PKA Size\n\nIt specifies the bits of N in PKA calculation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockSize {
    #[doc = "0: 512 bit"]
    B00 = 0,
    #[doc = "1: 1024 bit"]
    B01 = 1,
    #[doc = "2: 2048 bit"]
    B10 = 2,
}
impl From<BlockSize> for u8 {
    #[inline(always)]
    fn from(variant: BlockSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BlockSize {
    type Ux = u8;
}
#[doc = "Field `BLOCK_SIZE` reader - PKA Size\n\nIt specifies the bits of N in PKA calculation."]
pub type BlockSizeR = crate::FieldReader<BlockSize>;
impl BlockSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BlockSize> {
        match self.bits {
            0 => Some(BlockSize::B00),
            1 => Some(BlockSize::B01),
            2 => Some(BlockSize::B10),
            _ => None,
        }
    }
    #[doc = "512 bit"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == BlockSize::B00
    }
    #[doc = "1024 bit"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == BlockSize::B01
    }
    #[doc = "2048 bit"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == BlockSize::B10
    }
}
#[doc = "Field `BLOCK_SIZE` writer - PKA Size\n\nIt specifies the bits of N in PKA calculation."]
pub type BlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, BlockSize>;
impl<'a, REG> BlockSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 bit"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSize::B00)
    }
    #[doc = "1024 bit"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSize::B01)
    }
    #[doc = "2048 bit"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSize::B10)
    }
}
impl R {
    #[doc = "Bits 0:1 - PKA Size\n\nIt specifies the bits of N in PKA calculation."]
    #[inline(always)]
    pub fn block_size(&self) -> BlockSizeR {
        BlockSizeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PKA Size\n\nIt specifies the bits of N in PKA calculation."]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BlockSizeW<PkaCtrlSpec> {
        BlockSizeW::new(self, 0)
    }
}
#[doc = "PKA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaCtrlSpec;
impl crate::RegisterSpec for PkaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_ctrl::R`](R) reader structure"]
impl crate::Readable for PkaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pka_ctrl::W`](W) writer structure"]
impl crate::Writable for PkaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKA_CTRL to value 0"]
impl crate::Resettable for PkaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
