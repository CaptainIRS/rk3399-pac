#[doc = "Register `PERFCNT_SRC0` reader"]
pub type R = crate::R<PerfcntSrc0Spec>;
#[doc = "Register `PERFCNT_SRC0` writer"]
pub type W = crate::W<PerfcntSrc0Spec>;
#[doc = "This register holds all the possible source values for Performance\n\nCounter 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PerfcntSrc0 {
    #[doc = "0: disabled"]
    D0 = 0,
    #[doc = "1: total clock cycles"]
    D1 = 1,
    #[doc = "2: active clock cycles"]
    D2 = 2,
    #[doc = "3: read transactions, master"]
    D3 = 3,
    #[doc = "4: word reads, master"]
    D4 = 4,
    #[doc = "5: read transactions, slave"]
    D5 = 5,
    #[doc = "6: word reads, slave"]
    D6 = 6,
    #[doc = "7: read hit, slave"]
    D7 = 7,
    #[doc = "8: read misses, slave"]
    D8 = 8,
    #[doc = "9: read invalidates, slave"]
    D9 = 9,
    #[doc = "10: cacheable read transactions, slave"]
    D10 = 10,
    #[doc = "11: bad hit nmber, slave"]
    D11 = 11,
}
impl From<PerfcntSrc0> for u8 {
    #[inline(always)]
    fn from(variant: PerfcntSrc0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PerfcntSrc0 {
    type Ux = u8;
}
#[doc = "Field `PERFCNT_SRC0` reader - This register holds all the possible source values for Performance\n\nCounter 0"]
pub type PerfcntSrc0R = crate::FieldReader<PerfcntSrc0>;
impl PerfcntSrc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PerfcntSrc0> {
        match self.bits {
            0 => Some(PerfcntSrc0::D0),
            1 => Some(PerfcntSrc0::D1),
            2 => Some(PerfcntSrc0::D2),
            3 => Some(PerfcntSrc0::D3),
            4 => Some(PerfcntSrc0::D4),
            5 => Some(PerfcntSrc0::D5),
            6 => Some(PerfcntSrc0::D6),
            7 => Some(PerfcntSrc0::D7),
            8 => Some(PerfcntSrc0::D8),
            9 => Some(PerfcntSrc0::D9),
            10 => Some(PerfcntSrc0::D10),
            11 => Some(PerfcntSrc0::D11),
            _ => None,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == PerfcntSrc0::D0
    }
    #[doc = "total clock cycles"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == PerfcntSrc0::D1
    }
    #[doc = "active clock cycles"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == PerfcntSrc0::D2
    }
    #[doc = "read transactions, master"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == PerfcntSrc0::D3
    }
    #[doc = "word reads, master"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == PerfcntSrc0::D4
    }
    #[doc = "read transactions, slave"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == PerfcntSrc0::D5
    }
    #[doc = "word reads, slave"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == PerfcntSrc0::D6
    }
    #[doc = "read hit, slave"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == PerfcntSrc0::D7
    }
    #[doc = "read misses, slave"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == PerfcntSrc0::D8
    }
    #[doc = "read invalidates, slave"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == PerfcntSrc0::D9
    }
    #[doc = "cacheable read transactions, slave"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == PerfcntSrc0::D10
    }
    #[doc = "bad hit nmber, slave"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == PerfcntSrc0::D11
    }
}
#[doc = "Field `PERFCNT_SRC0` writer - This register holds all the possible source values for Performance\n\nCounter 0"]
pub type PerfcntSrc0W<'a, REG> = crate::FieldWriter<'a, REG, 7, PerfcntSrc0>;
impl<'a, REG> PerfcntSrc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D0)
    }
    #[doc = "total clock cycles"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D1)
    }
    #[doc = "active clock cycles"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D2)
    }
    #[doc = "read transactions, master"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D3)
    }
    #[doc = "word reads, master"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D4)
    }
    #[doc = "read transactions, slave"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D5)
    }
    #[doc = "word reads, slave"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D6)
    }
    #[doc = "read hit, slave"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D7)
    }
    #[doc = "read misses, slave"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D8)
    }
    #[doc = "read invalidates, slave"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D9)
    }
    #[doc = "cacheable read transactions, slave"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D10)
    }
    #[doc = "bad hit nmber, slave"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut crate::W<REG> {
        self.variant(PerfcntSrc0::D11)
    }
}
impl R {
    #[doc = "Bits 0:6 - This register holds all the possible source values for Performance\n\nCounter 0"]
    #[inline(always)]
    pub fn perfcnt_src0(&self) -> PerfcntSrc0R {
        PerfcntSrc0R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register holds all the possible source values for Performance\n\nCounter 0"]
    #[inline(always)]
    #[must_use]
    pub fn perfcnt_src0(&mut self) -> PerfcntSrc0W<PerfcntSrc0Spec> {
        PerfcntSrc0W::new(self, 0)
    }
}
#[doc = "performance counter 0 source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_src0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_src0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfcntSrc0Spec;
impl crate::RegisterSpec for PerfcntSrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfcnt_src0::R`](R) reader structure"]
impl crate::Readable for PerfcntSrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`perfcnt_src0::W`](W) writer structure"]
impl crate::Writable for PerfcntSrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERFCNT_SRC0 to value 0"]
impl crate::Resettable for PerfcntSrc0Spec {
    const RESET_VALUE: u32 = 0;
}
