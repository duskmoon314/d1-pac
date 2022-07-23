#[doc = "Register `asrcmbiststat` reader"]
pub struct R(crate::R<ASRCMBISTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASRCMBISTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASRCMBISTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASRCMBISTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asrcmbiststat` writer"]
pub struct W(crate::W<ASRCMBISTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASRCMBISTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ASRCMBISTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASRCMBISTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC MBIST Test Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrcmbiststat](index.html) module"]
pub struct ASRCMBISTSTAT_SPEC;
impl crate::RegisterSpec for ASRCMBISTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asrcmbiststat::R](R) reader structure"]
impl crate::Readable for ASRCMBISTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asrcmbiststat::W](W) writer structure"]
impl crate::Writable for ASRCMBISTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets asrcmbiststat to value 0"]
impl crate::Resettable for ASRCMBISTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
