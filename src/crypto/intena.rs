#[doc = "Register `INTENA` reader"]
pub type R = crate::R<IntenaSpec>;
#[doc = "Register `INTENA` writer"]
pub type W = crate::W<IntenaSpec>;
#[doc = "Set the interrupt Enable of block cipher DMA DONE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcdmaDoneEna {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<BcdmaDoneEna> for bool {
    #[inline(always)]
    fn from(variant: BcdmaDoneEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCDMA_DONE_ENA` reader - Set the interrupt Enable of block cipher DMA DONE"]
pub type BcdmaDoneEnaR = crate::BitReader<BcdmaDoneEna>;
impl BcdmaDoneEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcdmaDoneEna {
        match self.bits {
            true => BcdmaDoneEna::B1,
            false => BcdmaDoneEna::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcdmaDoneEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcdmaDoneEna::B0
    }
}
#[doc = "Field `BCDMA_DONE_ENA` writer - Set the interrupt Enable of block cipher DMA DONE"]
pub type BcdmaDoneEnaW<'a, REG> = crate::BitWriter<'a, REG, BcdmaDoneEna>;
impl<'a, REG> BcdmaDoneEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcdmaDoneEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcdmaDoneEna::B0)
    }
}
#[doc = "Set the interrupt Enable of block cipher DMA Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcdmaErrEna {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<BcdmaErrEna> for bool {
    #[inline(always)]
    fn from(variant: BcdmaErrEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCDMA_ERR_ENA` reader - Set the interrupt Enable of block cipher DMA Error"]
pub type BcdmaErrEnaR = crate::BitReader<BcdmaErrEna>;
impl BcdmaErrEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcdmaErrEna {
        match self.bits {
            true => BcdmaErrEna::B1,
            false => BcdmaErrEna::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcdmaErrEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcdmaErrEna::B0
    }
}
#[doc = "Field `BCDMA_ERR_ENA` writer - Set the interrupt Enable of block cipher DMA Error"]
pub type BcdmaErrEnaW<'a, REG> = crate::BitWriter<'a, REG, BcdmaErrEna>;
impl<'a, REG> BcdmaErrEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcdmaErrEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcdmaErrEna::B0)
    }
}
#[doc = "Set the interrupt Enable of hash receiving DMA DONE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HrdmaDoneEna {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<HrdmaDoneEna> for bool {
    #[inline(always)]
    fn from(variant: HrdmaDoneEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRDMA_DONE_ENA` reader - Set the interrupt Enable of hash receiving DMA DONE"]
pub type HrdmaDoneEnaR = crate::BitReader<HrdmaDoneEna>;
impl HrdmaDoneEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HrdmaDoneEna {
        match self.bits {
            true => HrdmaDoneEna::B1,
            false => HrdmaDoneEna::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HrdmaDoneEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HrdmaDoneEna::B0
    }
}
#[doc = "Field `HRDMA_DONE_ENA` writer - Set the interrupt Enable of hash receiving DMA DONE"]
pub type HrdmaDoneEnaW<'a, REG> = crate::BitWriter<'a, REG, HrdmaDoneEna>;
impl<'a, REG> HrdmaDoneEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HrdmaDoneEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HrdmaDoneEna::B0)
    }
}
#[doc = "Set the interrupt Enable of hash receiving DMA Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HrdmaErrEna {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<HrdmaErrEna> for bool {
    #[inline(always)]
    fn from(variant: HrdmaErrEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRDMA_ERR_ENA` reader - Set the interrupt Enable of hash receiving DMA Error"]
pub type HrdmaErrEnaR = crate::BitReader<HrdmaErrEna>;
impl HrdmaErrEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HrdmaErrEna {
        match self.bits {
            true => HrdmaErrEna::B1,
            false => HrdmaErrEna::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HrdmaErrEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HrdmaErrEna::B0
    }
}
#[doc = "Field `HRDMA_ERR_ENA` writer - Set the interrupt Enable of hash receiving DMA Error"]
pub type HrdmaErrEnaW<'a, REG> = crate::BitWriter<'a, REG, HrdmaErrEna>;
impl<'a, REG> HrdmaErrEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HrdmaErrEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HrdmaErrEna::B0)
    }
}
#[doc = "Set the interrupt Enable of hash done\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashDoneEna {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<HashDoneEna> for bool {
    #[inline(always)]
    fn from(variant: HashDoneEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_DONE_ENA` reader - Set the interrupt Enable of hash done"]
pub type HashDoneEnaR = crate::BitReader<HashDoneEna>;
impl HashDoneEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashDoneEna {
        match self.bits {
            true => HashDoneEna::B1,
            false => HashDoneEna::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HashDoneEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HashDoneEna::B0
    }
}
#[doc = "Field `HASH_DONE_ENA` writer - Set the interrupt Enable of hash done"]
pub type HashDoneEnaW<'a, REG> = crate::BitWriter<'a, REG, HashDoneEna>;
impl<'a, REG> HashDoneEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HashDoneEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HashDoneEna::B0)
    }
}
#[doc = "Set the interrupt Enable of PKA done\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkaDoneEna {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<PkaDoneEna> for bool {
    #[inline(always)]
    fn from(variant: PkaDoneEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKA_DONE_ENA` reader - Set the interrupt Enable of PKA done"]
pub type PkaDoneEnaR = crate::BitReader<PkaDoneEna>;
impl PkaDoneEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PkaDoneEna {
        match self.bits {
            true => PkaDoneEna::B1,
            false => PkaDoneEna::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PkaDoneEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PkaDoneEna::B0
    }
}
#[doc = "Field `PKA_DONE_ENA` writer - Set the interrupt Enable of PKA done"]
pub type PkaDoneEnaW<'a, REG> = crate::BitWriter<'a, REG, PkaDoneEna>;
impl<'a, REG> PkaDoneEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PkaDoneEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PkaDoneEna::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Set the interrupt Enable of block cipher DMA DONE"]
    #[inline(always)]
    pub fn bcdma_done_ena(&self) -> BcdmaDoneEnaR {
        BcdmaDoneEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the interrupt Enable of block cipher DMA Error"]
    #[inline(always)]
    pub fn bcdma_err_ena(&self) -> BcdmaErrEnaR {
        BcdmaErrEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the interrupt Enable of hash receiving DMA DONE"]
    #[inline(always)]
    pub fn hrdma_done_ena(&self) -> HrdmaDoneEnaR {
        HrdmaDoneEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set the interrupt Enable of hash receiving DMA Error"]
    #[inline(always)]
    pub fn hrdma_err_ena(&self) -> HrdmaErrEnaR {
        HrdmaErrEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set the interrupt Enable of hash done"]
    #[inline(always)]
    pub fn hash_done_ena(&self) -> HashDoneEnaR {
        HashDoneEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set the interrupt Enable of PKA done"]
    #[inline(always)]
    pub fn pka_done_ena(&self) -> PkaDoneEnaR {
        PkaDoneEnaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set the interrupt Enable of block cipher DMA DONE"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_done_ena(&mut self) -> BcdmaDoneEnaW<IntenaSpec> {
        BcdmaDoneEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Set the interrupt Enable of block cipher DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_err_ena(&mut self) -> BcdmaErrEnaW<IntenaSpec> {
        BcdmaErrEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Set the interrupt Enable of hash receiving DMA DONE"]
    #[inline(always)]
    #[must_use]
    pub fn hrdma_done_ena(&mut self) -> HrdmaDoneEnaW<IntenaSpec> {
        HrdmaDoneEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Set the interrupt Enable of hash receiving DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn hrdma_err_ena(&mut self) -> HrdmaErrEnaW<IntenaSpec> {
        HrdmaErrEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - Set the interrupt Enable of hash done"]
    #[inline(always)]
    #[must_use]
    pub fn hash_done_ena(&mut self) -> HashDoneEnaW<IntenaSpec> {
        HashDoneEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - Set the interrupt Enable of PKA done"]
    #[inline(always)]
    #[must_use]
    pub fn pka_done_ena(&mut self) -> PkaDoneEnaW<IntenaSpec> {
        PkaDoneEnaW::new(self, 5)
    }
}
#[doc = "Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenaSpec;
impl crate::RegisterSpec for IntenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intena::R`](R) reader structure"]
impl crate::Readable for IntenaSpec {}
#[doc = "`write(|w| ..)` method takes [`intena::W`](W) writer structure"]
impl crate::Writable for IntenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENA to value 0"]
impl crate::Resettable for IntenaSpec {
    const RESET_VALUE: u32 = 0;
}
