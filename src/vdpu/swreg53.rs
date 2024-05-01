#[doc = "Register `SWREG53` reader"]
pub type R = crate::R<Swreg53Spec>;
#[doc = "Register `SWREG53` writer"]
pub type W = crate::W<Swreg53Spec>;
#[doc = "the dec format select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDecFmtSel {
    #[doc = "0: H.264,"]
    D0 = 0,
    #[doc = "1: MPEG-4,"]
    D1 = 1,
    #[doc = "2: H.263,"]
    D2 = 2,
    #[doc = "3: JPEG,"]
    D3 = 3,
    #[doc = "5: MPEG-2,"]
    D5 = 5,
    #[doc = "6: MPEG-1,"]
    D6 = 6,
    #[doc = "7: VP6,"]
    D7 = 7,
    #[doc = "8: RV,"]
    D8 = 8,
    #[doc = "9: VP7,"]
    D9 = 9,
    #[doc = "11: AVS, others : reserved"]
    D11 = 11,
}
impl From<SwDecFmtSel> for u8 {
    #[inline(always)]
    fn from(variant: SwDecFmtSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDecFmtSel {
    type Ux = u8;
}
#[doc = "Field `SW_DEC_FMT_SEL` reader - the dec format select"]
pub type SwDecFmtSelR = crate::FieldReader<SwDecFmtSel>;
impl SwDecFmtSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDecFmtSel> {
        match self.bits {
            0 => Some(SwDecFmtSel::D0),
            1 => Some(SwDecFmtSel::D1),
            2 => Some(SwDecFmtSel::D2),
            3 => Some(SwDecFmtSel::D3),
            5 => Some(SwDecFmtSel::D5),
            6 => Some(SwDecFmtSel::D6),
            7 => Some(SwDecFmtSel::D7),
            8 => Some(SwDecFmtSel::D8),
            9 => Some(SwDecFmtSel::D9),
            11 => Some(SwDecFmtSel::D11),
            _ => None,
        }
    }
    #[doc = "H.264,"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwDecFmtSel::D0
    }
    #[doc = "MPEG-4,"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwDecFmtSel::D1
    }
    #[doc = "H.263,"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwDecFmtSel::D2
    }
    #[doc = "JPEG,"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwDecFmtSel::D3
    }
    #[doc = "MPEG-2,"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == SwDecFmtSel::D5
    }
    #[doc = "MPEG-1,"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == SwDecFmtSel::D6
    }
    #[doc = "VP6,"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == SwDecFmtSel::D7
    }
    #[doc = "RV,"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == SwDecFmtSel::D8
    }
    #[doc = "VP7,"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == SwDecFmtSel::D9
    }
    #[doc = "AVS, others : reserved"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == SwDecFmtSel::D11
    }
}
#[doc = "Field `SW_DEC_FMT_SEL` writer - the dec format select"]
pub type SwDecFmtSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, SwDecFmtSel>;
impl<'a, REG> SwDecFmtSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "H.264,"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D0)
    }
    #[doc = "MPEG-4,"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D1)
    }
    #[doc = "H.263,"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D2)
    }
    #[doc = "JPEG,"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D3)
    }
    #[doc = "MPEG-2,"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D5)
    }
    #[doc = "MPEG-1,"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D6)
    }
    #[doc = "VP6,"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D7)
    }
    #[doc = "RV,"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D8)
    }
    #[doc = "VP7,"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D9)
    }
    #[doc = "AVS, others : reserved"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFmtSel::D11)
    }
}
impl R {
    #[doc = "Bits 0:3 - the dec format select"]
    #[inline(always)]
    pub fn sw_dec_fmt_sel(&self) -> SwDecFmtSelR {
        SwDecFmtSelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - the dec format select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_fmt_sel(&mut self) -> SwDecFmtSelW<Swreg53Spec> {
        SwDecFmtSelW::new(self, 0)
    }
}
#[doc = "decoder format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg53Spec;
impl crate::RegisterSpec for Swreg53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg53::R`](R) reader structure"]
impl crate::Readable for Swreg53Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg53::W`](W) writer structure"]
impl crate::Writable for Swreg53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG53 to value 0"]
impl crate::Resettable for Swreg53Spec {
    const RESET_VALUE: u32 = 0;
}
