#[doc = "Register `LINK_BW_SET` reader"]
pub type R = crate::R<LinkBwSetSpec>;
#[doc = "Register `LINK_BW_SET` writer"]
pub type W = crate::W<LinkBwSetSpec>;
#[doc = "Main link bandwidth setting:\n\nValue on reset: 10"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LinkBwSet {
    #[doc = "6: 1.62Gpbs per lane"]
    H06 = 6,
    #[doc = "10: 2.7Gpbs per lane other: Reserved"]
    H0a = 10,
}
impl From<LinkBwSet> for u8 {
    #[inline(always)]
    fn from(variant: LinkBwSet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LinkBwSet {
    type Ux = u8;
}
#[doc = "Field `LINK_BW_SET` reader - Main link bandwidth setting:"]
pub type LinkBwSetR = crate::FieldReader<LinkBwSet>;
impl LinkBwSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LinkBwSet> {
        match self.bits {
            6 => Some(LinkBwSet::H06),
            10 => Some(LinkBwSet::H0a),
            _ => None,
        }
    }
    #[doc = "1.62Gpbs per lane"]
    #[inline(always)]
    pub fn is_h06(&self) -> bool {
        *self == LinkBwSet::H06
    }
    #[doc = "2.7Gpbs per lane other: Reserved"]
    #[inline(always)]
    pub fn is_h0a(&self) -> bool {
        *self == LinkBwSet::H0a
    }
}
#[doc = "Field `LINK_BW_SET` writer - Main link bandwidth setting:"]
pub type LinkBwSetW<'a, REG> = crate::FieldWriter<'a, REG, 4, LinkBwSet>;
impl<'a, REG> LinkBwSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.62Gpbs per lane"]
    #[inline(always)]
    pub fn h06(self) -> &'a mut crate::W<REG> {
        self.variant(LinkBwSet::H06)
    }
    #[doc = "2.7Gpbs per lane other: Reserved"]
    #[inline(always)]
    pub fn h0a(self) -> &'a mut crate::W<REG> {
        self.variant(LinkBwSet::H0a)
    }
}
impl R {
    #[doc = "Bits 0:3 - Main link bandwidth setting:"]
    #[inline(always)]
    pub fn link_bw_set(&self) -> LinkBwSetR {
        LinkBwSetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Main link bandwidth setting:"]
    #[inline(always)]
    #[must_use]
    pub fn link_bw_set(&mut self) -> LinkBwSetW<LinkBwSetSpec> {
        LinkBwSetW::new(self, 0)
    }
}
#[doc = "Main Link Bandwidth Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_bw_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_bw_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkBwSetSpec;
impl crate::RegisterSpec for LinkBwSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_bw_set::R`](R) reader structure"]
impl crate::Readable for LinkBwSetSpec {}
#[doc = "`write(|w| ..)` method takes [`link_bw_set::W`](W) writer structure"]
impl crate::Writable for LinkBwSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINK_BW_SET to value 0x0a"]
impl crate::Resettable for LinkBwSetSpec {
    const RESET_VALUE: u32 = 0x0a;
}
